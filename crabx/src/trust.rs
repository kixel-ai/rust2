use crate::fetchers::Receipt;
use crate::network;
use anyhow::{Context, Result, bail, ensure};
use semver::Version;
use serde::Deserialize;
use sigstore_verify::{
	VerificationPolicy, Verifier,
	trust_root::{TrustedRoot, TufConfig},
	types::{Bundle, Sha256Hash, SignatureContent},
};
use std::{fs::File, io::BufReader, path::Path};

#[derive(Deserialize)]
struct Page {
	#[serde(deserialize_with = "one_attestation")]
	attestations: Option<Attestation>,
}

#[derive(Deserialize)]
struct Attestation {
	bundle: Option<Bundle>,
	bundle_url: Option<reqwest::Url>,
}

fn one_attestation<'de, D: serde::Deserializer<'de>>(
	deserializer: D,
) -> std::result::Result<Option<Attestation>, D::Error> {
	struct One;
	impl<'de> serde::de::Visitor<'de> for One {
		type Value = Option<Attestation>;
		fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.write_str("a page containing at most one attestation")
		}
		fn visit_seq<A: serde::de::SeqAccess<'de>>(self, mut seq: A) -> std::result::Result<Self::Value, A::Error> {
			let first = seq.next_element()?;
			if seq.next_element::<serde::de::IgnoredAny>()?.is_some() {
				return Err(serde::de::Error::custom("GitHub ignored the attestation page size"));
			}
			Ok(first)
		}
	}
	deserializer.deserialize_seq(One)
}

pub async fn verify(receipt: &Receipt, version: &Version, stage: &Path) -> Result<String> {
	let Some(repo) = receipt.repository.as_deref().map(repository).transpose()?.flatten() else {
		return Ok("author release (repository does not support GitHub attestations)".into());
	};
	let digest = hex::encode(receipt.sha256);
	let client = network::client()?;
	// Request one bundle per page. An attestation for a different digest is never queried.
	let endpoint = format!("https://api.github.com/repos/{repo}/attestations/sha256:{digest}");
	let mut url = reqwest::Url::parse(&format!("{endpoint}?per_page=1"))?;
	let token = github_token()?;
	let mut first_page = true;
	let mut verified = false;
	let mut roots: [Option<TrustedRoot>; 2] = [None, None];
	loop {
		let mut request = client.get(url.clone()).header("Accept", "application/vnd.github+json");
		if let Some(token) = &token {
			request = request.bearer_auth(token);
		}
		let response = request.send().await?;
		ensure!(
			response.url().host_str() == Some("api.github.com"),
			"GitHub API redirected outside GitHub"
		);
		// GitHub returns 404 when there are no attestations for this digest.
		if response.status() == reqwest::StatusCode::NOT_FOUND && first_page {
			return Ok("author release (no published attestation)".into());
		}
		let response = response.error_for_status().context("query GitHub attestations")?;
		let next = response.headers().get("link").map(|v| v.to_str()).transpose()?;
		let next = next_page(next, &url, &repo, &digest)?;
		let file = network::spool(response, stage).await?;
		let page: Page = serde_json::from_reader(BufReader::new(File::open(file.path())?))?;
		if page.attestations.is_none() {
			ensure!(next.is_none(), "empty attestation page has a next page");
			break;
		}
		if let Some(attestation) = page.attestations {
			let bundle = match attestation.bundle {
				Some(bundle) => bundle,
				None => {
					let url = attestation
						.bundle_url
						.context("attestation has neither bundle nor bundle_url")?;
					ensure!(
						matches!(url.scheme(), "https" | "http")
							&& url.username().is_empty()
							&& url.password().is_none(),
						"invalid attestation bundle URL"
					);
					// External bundle storage receives no GitHub credential.
					let response = client.get(url).send().await?.error_for_status()?;
					let file = network::spool(response, stage).await?;
					external_bundle(file.path())?
				}
			};
			// GitHub's timestamp-only trust domain has its own CA and TSA. A
			// public-good bundle cannot lose its log and verify under that CA.
			let github = bundle.verification_material.tlog_entries.is_empty();
			let root = &mut roots[usize::from(github)];
			if root.is_none() {
				*root = Some(if github {
					TrustedRoot::from_tuf(TufConfig::github()).await?
				} else {
					TrustedRoot::production().await?
				});
			}
			let verifier = Verifier::new(root.as_ref().context("no Sigstore trust root")?)?;
			verify_bundle(
				&verifier,
				&bundle,
				Artifact {
					digest: &digest,
					repo: &repo,
					version,
					asset: &receipt.url,
				},
			)?;
			verified = true;
		}
		let Some(next) = next else { break };
		url = next;
		first_page = false;
	}
	if verified {
		Ok(format!("GitHub attestation sha256:{digest}"))
	} else {
		Ok("author release (no published attestation)".into())
	}
}

// Raw Snappy copy tags expand at most 64 bytes per tag. Bound allocation by
// the compressed input, rather than trusting its advertised output length.
fn external_bundle(path: &Path) -> Result<Bundle> {
	let compressed = std::fs::read(path)?;
	let length = snap::raw::decompress_len(&compressed)?;
	ensure!(
		length <= compressed.len().saturating_mul(64),
		"invalid Snappy expansion"
	);
	let json = snap::raw::Decoder::new().decompress_vec(&compressed)?;
	Ok(serde_json::from_slice(&json)?)
}

// Tokens are sent only to the fixed, validated GitHub API origin, never to
// archive hosts, sparse indexes, or Sigstore's trust-root services.
fn github_token() -> Result<Option<String>> {
	for name in ["GH_TOKEN", "GITHUB_TOKEN"] {
		match std::env::var(name) {
			Ok(token) if !token.is_empty() => return Ok(Some(token)),
			Ok(_) | Err(std::env::VarError::NotPresent) => (),
			Err(_) => bail!("{name} is not UTF-8"),
		}
	}
	Ok(None)
}

fn next_page(link: Option<&str>, current: &reqwest::Url, repo: &str, digest: &str) -> Result<Option<reqwest::Url>> {
	let mut next = None;
	for part in link.into_iter().flat_map(|link| link.split(',')) {
		let mut fields = part.split(';');
		let target = fields.next().context("missing pagination URL")?.trim();
		if !fields.any(|field| field.trim() == "rel=\"next\"") {
			continue;
		}
		ensure!(next.is_none(), "duplicate next attestation page");
		let target = target
			.strip_prefix('<')
			.and_then(|s| s.strip_suffix('>'))
			.context("invalid pagination URL")?;
		let url = reqwest::Url::parse(target)?;
		let suffix = format!("/attestations/sha256:{digest}");
		let path = url
			.path()
			.strip_suffix(&suffix)
			.context("pagination changed the artifact digest")?;
		// GitHub canonicalizes /repos/owner/name to /repositories/<numeric ID>
		// in Link headers. Signed identities remain bound to the original repo.
		let canonical = path
			.strip_prefix("/repositories/")
			.is_some_and(|id| !id.is_empty() && id.bytes().all(|b| b.is_ascii_digit()));
		ensure!(
			path == format!("/repos/{repo}") || canonical,
			"invalid attestation pagination path"
		);
		ensure!(
			url.scheme() == "https"
				&& url.host_str() == Some("api.github.com")
				&& url.port().is_none()
				&& url.username().is_empty()
				&& url.password().is_none()
				&& url.fragment().is_none(),
			"invalid attestation pagination origin"
		);
		ensure!(
			url.query_pairs()
				.filter(|(key, _)| key == "per_page")
				.map(|(_, value)| value)
				.collect::<Vec<_>>()
				== ["1"],
			"pagination changed the attestation page size"
		);
		ensure!(url != *current, "GitHub repeated the attestation page");
		next = Some(url);
	}
	Ok(next)
}

#[derive(Clone, Copy)]
struct Artifact<'a> {
	digest: &'a str,
	repo: &'a str,
	version: &'a Version,
	asset: &'a reqwest::Url,
}

fn verify_bundle(verifier: &Verifier, bundle: &Bundle, artifact: Artifact<'_>) -> Result<()> {
	let github = bundle.verification_material.tlog_entries.is_empty();
	let release = release(bundle)?;
	let mut policy = if release.is_some() {
		ensure!(github, "GitHub release attestation uses the wrong trust domain");
		VerificationPolicy::any_identity().require_identity("https://dotcom.releases.github.com")
	} else {
		VerificationPolicy::any_identity().require_issuer("https://token.actions.githubusercontent.com")
	};
	if github {
		// This CA issues timestamp-only bundles without public CT or Rekor.
		// The verifier still requires a verified RFC 3161 time, CA chain,
		// signature and artifact digest. Never apply this policy to the
		// public-good root in the download path.
		policy = policy.skip_tlog_unsafe().skip_sct();
	}
	let result = verifier
		.verify(Sha256Hash::from_hex(artifact.digest)?, bundle, &policy)
		.context("invalid published attestation")?;
	ensure!(
		result.certificate_verified()
			&& !result.verified_timestamps().is_empty()
			&& (github || (result.sct_verified() && result.tlog_verified())),
		"attestation verification is incomplete"
	);
	if let Some(release) = release {
		ensure!(
			release.repository.eq_ignore_ascii_case(artifact.repo),
			"release attestation names another repository"
		);
		return artifact.tag(&release.tag);
	}
	let cert = result.certificate().context("attestation has no certificate")?;
	artifact.authorize(
		&cert.ci_claims,
		cert.identity.as_ref().map(|identity| identity.as_str()),
	)
}

#[derive(Deserialize)]
struct Release {
	repository: String,
	tag: String,
}

fn release(bundle: &Bundle) -> Result<Option<Release>> {
	let SignatureContent::DsseEnvelope(envelope) = &bundle.content else {
		return Ok(None);
	};
	ensure!(
		envelope.payload_type == "application/vnd.in-toto+json",
		"unsupported attestation payload type"
	);
	#[derive(Deserialize)]
	struct Header {
		#[serde(rename = "predicateType")]
		kind: String,
	}
	let header: Header = serde_json::from_slice(envelope.payload.as_bytes())?;
	if header.kind != "https://in-toto.io/attestation/release/v0.2" {
		return Ok(None);
	}
	#[derive(Deserialize)]
	struct Statement {
		predicate: Release,
	}
	let statement: Statement = serde_json::from_slice(envelope.payload.as_bytes())?;
	Ok(Some(statement.predicate))
}

fn repository(value: &str) -> Result<Option<String>> {
	let url = reqwest::Url::parse(value).context("invalid crate repository URL")?;
	if !matches!(url.host_str(), Some("github.com" | "www.github.com")) {
		return Ok(None);
	}
	ensure!(
		matches!(url.scheme(), "https" | "http") && url.username().is_empty() && url.password().is_none(),
		"invalid GitHub repository URL"
	);
	let mut segments = url.path_segments().context("repository has no path")?;
	let owner = segments.next().context("repository has no owner")?;
	let repo = segments
		.next()
		.context("repository has no name")?
		.trim_end_matches(".git");
	ensure!(!owner.is_empty() && !repo.is_empty(), "repository has no owner or name");
	Ok(Some(format!("{owner}/{repo}").to_ascii_lowercase()))
}

impl Artifact<'_> {
	fn authorize(&self, claims: &sigstore_verify::crypto::FulcioCiClaims, identity: Option<&str>) -> Result<()> {
		let repo = self.repo;
		let expected = format!("https://github.com/{repo}");
		let matches = match claims.source_repository_uri.as_deref() {
			Some(uri) => uri.eq_ignore_ascii_case(&expected),
			None => claims
				.deprecated_github
				.workflow_repository
				.as_deref()
				.is_some_and(|value| value.eq_ignore_ascii_case(repo)),
		};
		ensure!(matches, "attestation was issued for a different repository");
		let signer = claims
			.build_signer_uri
			.as_deref()
			.or(identity)
			.context("attestation lacks a workflow identity")?;
		let signer = signer.to_ascii_lowercase();
		let own = signer.starts_with(&format!("{expected}/.github/workflows/"));
		let pipeline = signer.starts_with("https://github.com/kixel-ai/rust2/.github/workflows/");
		ensure!(
			own || pipeline,
			"attestation signer is neither the author's workflow nor the crabx pipeline"
		);
		let reference = claims
			.source_repository_ref
			.as_deref()
			.or(claims.deprecated_github.workflow_ref.as_deref())
			.context("attestation lacks a source ref")?;
		if let Some(tag) = reference.strip_prefix("refs/tags/") {
			self.tag(tag)?;
		} else if !reference.starts_with("refs/heads/") {
			bail!("attestation was issued from an untrusted ref {reference:?}");
		}
		Ok(())
	}

	fn tag(&self, tag: &str) -> Result<()> {
		let asset_tag = self
			.asset
			.path()
			.split("/releases/download/")
			.nth(1)
			.and_then(|v| v.split('/').next())
			.map(|v| percent_encoding::percent_decode_str(v).decode_utf8())
			.transpose()?;
		ensure!(
			asset_tag.as_deref() == Some(tag) || tag == self.version.to_string() || tag == format!("v{}", self.version),
			"attestation is for another release tag"
		);
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn external_bundles_decode_raw_snappy_and_reject_corruption() {
		let temp = tempfile::NamedTempFile::new().unwrap();
		let json = include_bytes!("../tests/data/github-attestation.json");
		let compressed = snap::raw::Encoder::new().compress_vec(json).unwrap();
		std::fs::write(temp.path(), compressed).unwrap();
		external_bundle(temp.path()).unwrap();
		std::fs::write(temp.path(), b"invalid snappy").unwrap();
		assert!(external_bundle(temp.path()).is_err());
	}

	#[test]
	fn github_cursor_pagination_preserves_the_cursor_and_api_boundary() {
		let current =
			reqwest::Url::parse("https://api.github.com/repos/a/b/attestations/sha256:abc?per_page=1").unwrap();
		let target = "https://api.github.com/repositories/192136193/attestations/sha256:abc?per_page=1&after=58893410";
		let link = format!("<{target}>; rel=\"next\", <https://docs.github.com/api>; rel=\"deprecation\"");
		assert_eq!(
			next_page(Some(&link), &current, "a/b", "abc")
				.unwrap()
				.unwrap()
				.as_str(),
			target
		);
		assert!(next_page(None, &current, "a/b", "abc").unwrap().is_none());
		for bad in [
			target.replace("api.github.com", "evil.example"),
			target.replace("https:", "http:"),
			target.replace("api.github.com", "user@api.github.com"),
			target.replace("sha256:abc", "sha256:def"),
			target.replace("per_page=1", "per_page=100"),
			target.replace("192136193", "a/b"),
			current.to_string(),
		] {
			assert!(
				next_page(Some(&format!("<{bad}>; rel=\"next\"")), &current, "a/b", "abc").is_err(),
				"{bad}"
			);
		}
	}

	#[test]
	fn github_release_checks_tsa_digest_repository_tag_and_trust_domain() {
		let root = TrustedRoot::from_embedded(sigstore_verify::trust_root::SigstoreInstance::GitHub).unwrap();
		let verifier = Verifier::new(&root).unwrap();
		let json = include_str!("../tests/data/github-attestation.json");
		let bundle = Bundle::from_json(json).unwrap();
		let version = Version::new(0, 1, 9);
		let asset =
			reqwest::Url::parse("https://github.com/jdx/communique/releases/download/v0.1.9/asset.tgz").unwrap();
		let artifact = Artifact {
			digest: "76f1fe8593bf227cca2c089e3c16dc95014a8d3e89c5dd220530469ca043c428",
			repo: "jdx/communique",
			version: &version,
			asset: &asset,
		};
		verify_bundle(&verifier, &bundle, artifact).unwrap();
		assert!(
			verify_bundle(
				&verifier,
				&bundle,
				Artifact {
					repo: "other/repo",
					..artifact
				}
			)
			.is_err()
		);
		assert!(
			verify_bundle(
				&verifier,
				&bundle,
				Artifact {
					digest: &"00".repeat(32),
					..artifact
				}
			)
			.is_err()
		);
		let other_version = Version::new(9, 9, 9);
		let other_asset =
			reqwest::Url::parse("https://github.com/jdx/communique/releases/download/v9.9.9/asset.tgz").unwrap();
		assert!(
			verify_bundle(
				&verifier,
				&bundle,
				Artifact {
					version: &other_version,
					asset: &other_asset,
					..artifact
				}
			)
			.is_err()
		);
		let public = TrustedRoot::from_json(include_str!("../tests/data/trusted-root.json")).unwrap();
		assert!(verify_bundle(&Verifier::new(&public).unwrap(), &bundle, artifact).is_err());
		let mut changed: serde_json::Value = serde_json::from_str(json).unwrap();
		let token = &mut changed["verificationMaterial"]["timestampVerificationData"]["rfc3161Timestamps"][0]["signedTimestamp"];
		let mut signature = token.as_str().unwrap().to_owned();
		let index = signature.len() - 8;
		let replacement = if signature.as_bytes()[index] == b'A' { "B" } else { "A" };
		signature.replace_range(index..index + 1, replacement);
		*token = signature.into();
		let changed = Bundle::from_json(&serde_json::to_string(&changed).unwrap()).unwrap();
		assert!(verify_bundle(&verifier, &changed, artifact).is_err());
	}

	#[test]
	fn repositories_are_canonical_and_identity_is_bound() {
		assert_eq!(
			repository("https://github.com/moq-dev/moq.git").unwrap().as_deref(),
			Some("moq-dev/moq")
		);
		assert_eq!(
			repository("https://github.com/moq-dev/moq/tree/main/rs/moq-cli")
				.unwrap()
				.as_deref(),
			Some("moq-dev/moq")
		);
		assert!(repository("https://gitlab.com/a/b").unwrap().is_none());
		assert!(repository("https://github.com/a").is_err());
		assert_eq!(repository("http://github.com/a/b").unwrap().as_deref(), Some("a/b"));
		assert_eq!(
			repository("https://www.github.com/a/b").unwrap().as_deref(),
			Some("a/b")
		);
		let mut claims = sigstore_verify::crypto::FulcioCiClaims::default();
		claims.source_repository_uri = Some("https://github.com/a/b".into());
		claims.build_signer_uri = Some("https://github.com/a/b/.github/workflows/release.yml@refs/tags/v1.2.3".into());
		claims.source_repository_ref = Some("refs/tags/v1.2.3".into());
		let asset = reqwest::Url::parse("https://github.com/a/b/releases/download/v1.2.3/asset.tgz").unwrap();
		let version = Version::new(1, 2, 3);
		assert!(
			Artifact {
				digest: "",
				repo: "a/b",
				version: &version,
				asset: &asset
			}
			.authorize(&claims, None)
			.is_ok()
		);
		assert!(
			Artifact {
				digest: "",
				repo: "a/evil",
				version: &version,
				asset: &asset
			}
			.authorize(&claims, None)
			.is_err()
		);
		claims.source_repository_ref = Some("refs/pull/1/merge".into());
		assert!(
			Artifact {
				digest: "",
				repo: "a/b",
				version: &version,
				asset: &asset
			}
			.authorize(&claims, None)
			.is_err()
		);
	}

	#[test]
	fn legacy_fulcio_claims_still_bind_repository_workflow_and_ref() {
		let mut claims = sigstore_verify::crypto::FulcioCiClaims::default();
		claims.deprecated_github.workflow_repository = Some("a/b".into());
		claims.deprecated_github.workflow_ref = Some("refs/tags/v1.2.3".into());
		let version = Version::new(1, 2, 3);
		let asset = reqwest::Url::parse("https://github.com/a/b/releases/download/v1.2.3/asset.tgz").unwrap();
		let artifact = Artifact {
			digest: "",
			repo: "a/b",
			version: &version,
			asset: &asset,
		};
		let identity = Some("https://github.com/a/b/.github/workflows/release.yml@refs/tags/v1.2.3");
		artifact.authorize(&claims, identity).unwrap();
		assert!(artifact.authorize(&claims, None).is_err());
		assert!(
			artifact
				.authorize(
					&claims,
					Some("https://github.com/evil/fork/.github/workflows/release.yml")
				)
				.is_err()
		);
		claims.deprecated_github.workflow_repository = Some("evil/fork".into());
		assert!(artifact.authorize(&claims, identity).is_err());
		claims.deprecated_github.workflow_repository = Some("a/b".into());
		claims.deprecated_github.workflow_ref = Some("refs/pull/1/merge".into());
		assert!(artifact.authorize(&claims, identity).is_err());
	}

	#[test]
	fn real_attestation_rejects_changed_digest_identity_signature_and_log() {
		let root = TrustedRoot::from_json(include_str!("../tests/data/trusted-root.json")).unwrap();
		let verifier = Verifier::new(&root).unwrap();
		let json = include_str!("../tests/data/attestation.json");
		let bundle = Bundle::from_json(json).unwrap();
		let digest = "54303491a8418fbed24344b513546182c29b43bf282ceb433af65e2299f9271f";
		let version = Version::new(1, 2, 3);
		let url =
			reqwest::Url::parse("https://github.com/prefix-dev/sigstore-example/releases/download/v1.2.3/asset.tgz")
				.unwrap();
		assert!(
			verify_bundle(
				&verifier,
				&bundle,
				Artifact {
					digest,
					repo: "prefix-dev/sigstore-example",
					version: &version,
					asset: &url
				}
			)
			.is_ok()
		);
		assert!(
			verify_bundle(
				&verifier,
				&bundle,
				Artifact {
					digest: &"00".repeat(32),
					repo: "prefix-dev/sigstore-example",
					version: &version,
					asset: &url
				}
			)
			.is_err()
		);
		assert!(
			verify_bundle(
				&verifier,
				&bundle,
				Artifact {
					digest,
					repo: "evil/fork",
					version: &version,
					asset: &url
				}
			)
			.is_err()
		);
		let mut tampered: serde_json::Value = serde_json::from_str(json).unwrap();
		tampered["dsseEnvelope"]["signatures"][0]["sig"] = "AAAA".into();
		let tampered = Bundle::from_json(&serde_json::to_string(&tampered).unwrap()).unwrap();
		assert!(
			verify_bundle(
				&verifier,
				&tampered,
				Artifact {
					digest,
					repo: "prefix-dev/sigstore-example",
					version: &version,
					asset: &url
				}
			)
			.is_err()
		);
		let mut tampered: serde_json::Value = serde_json::from_str(json).unwrap();
		tampered["verificationMaterial"]["tlogEntries"][0]["integratedTime"] = "1".into();
		let tampered = Bundle::from_json(&serde_json::to_string(&tampered).unwrap()).unwrap();
		assert!(
			verify_bundle(
				&verifier,
				&tampered,
				Artifact {
					digest,
					repo: "prefix-dev/sigstore-example",
					version: &version,
					asset: &url
				}
			)
			.is_err()
		);
	}
}
