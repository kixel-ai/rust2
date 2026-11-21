use itertools::Itertools;
use leon::{Item, Template};
use leon_macros::template;
use url::Url;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RepositoryHost {
	GitHub,
	GitLab,
	BitBucket,
	SourceForge,
	Codeberg,
	Unknown,
}

/// Make sure to update `binstalk_bins::possible_bin_dirs`
/// if you modified FULL_FILENAMES or NOVERSION_FILENAMES.
pub const FULL_FILENAMES: &[Template<'_>] = &[
	template!("/{ name }-{ target }-v{ version }{ archive-suffix }"),
	template!("/{ name }-{ target }-{ version }{ archive-suffix }"),
	template!("/{ name }-{ version }-{ target }{ archive-suffix }"),
	template!("/{ name }-v{ version }-{ target }{ archive-suffix }"),
	template!("/{ name }_{ target }_v{ version }{ archive-suffix }"),
	template!("/{ name }_{ target }_{ version }{ archive-suffix }"),
	template!("/{ name }_{ version }_{ target }{ archive-suffix }"),
	template!("/{ name }_v{ version }_{ target }{ archive-suffix }"),
];

pub const NOVERSION_FILENAMES: &[Template<'_>] = &[
	template!("/{ name }-{ target }{ archive-suffix }"),
	template!("/{ name }_{ target }{ archive-suffix }"),
];

const GITHUB_RELEASE_PATHS: &[Template<'_>] = &[
	template!("{ repo }/releases/download/{ version }"),
	template!("{ repo }/releases/download/v{ version }"),
	// %2F is escaped form of '/'
	template!("{ repo }/releases/download/{ subcrate }%2F{ version }"),
	template!("{ repo }/releases/download/{ subcrate }%2Fv{ version }"),
	// Default tags of workspace release tools such as release-plz and cargo-release.
	template!("{ repo }/releases/download/{ name }-v{ version }"),
	template!("{ repo }/releases/download/{ name }-{ version }"),
];

const GITLAB_RELEASE_PATHS: &[Template<'_>] = &[
	template!("{ repo }/-/releases/{ version }/downloads/binaries"),
	template!("{ repo }/-/releases/v{ version }/downloads/binaries"),
	// %2F is escaped form of '/'
	template!("{ repo }/-/releases/{ subcrate }%2F{ version }/downloads/binaries"),
	template!("{ repo }/-/releases/{ subcrate }%2Fv{ version }/downloads/binaries"),
];

const BITBUCKET_RELEASE_PATHS: &[Template<'_>] = &[template!("{ repo }/downloads")];

// No `{ subcrate }` paths: `RepoInfo::detect_subcrate` never finds a subcrate
// in a SourceForge URL, so they would always be filtered out.
const SOURCEFORGE_RELEASE_PATHS: &[Template<'_>] = &[
	template!("{ repo }/files/binaries/{ version }"),
	template!("{ repo }/files/binaries/v{ version }"),
];

/// Every filename is appended to every release path, followed by `suffix`.
struct DefaultPkgUrls {
	paths: &'static [Template<'static>],
	filenames: &'static [&'static [Template<'static>]],
	suffix: &'static str,
}

impl RepositoryHost {
	pub fn guess_git_hosting_services(repo: &Url) -> Self {
		use RepositoryHost::*;

		match repo.domain() {
			Some(domain) if domain.starts_with("github") => GitHub,
			Some(domain) if domain.starts_with("gitlab") => GitLab,
			Some("bitbucket.org") => BitBucket,
			Some("sourceforge.net") => SourceForge,
			Some("codeberg.org") => Codeberg,
			_ => Unknown,
		}
	}

	/// The parts that make up the default `pkg-url` templates of this host.
	///
	/// SUPPORT.md documents these, see `tests::support_md`.
	fn default_pkg_urls(self) -> Option<DefaultPkgUrls> {
		use RepositoryHost::*;

		Some(match self {
			GitHub => DefaultPkgUrls {
				paths: GITHUB_RELEASE_PATHS,
				filenames: &[FULL_FILENAMES, NOVERSION_FILENAMES],
				suffix: "",
			},
			GitLab => DefaultPkgUrls {
				paths: GITLAB_RELEASE_PATHS,
				filenames: &[FULL_FILENAMES, NOVERSION_FILENAMES],
				suffix: "",
			},
			BitBucket => DefaultPkgUrls {
				paths: BITBUCKET_RELEASE_PATHS,
				filenames: &[FULL_FILENAMES],
				suffix: "",
			},
			SourceForge => DefaultPkgUrls {
				paths: SOURCEFORGE_RELEASE_PATHS,
				filenames: &[FULL_FILENAMES, NOVERSION_FILENAMES],
				suffix: "/download",
			},
			// Codeberg (Forgejo) has the same release paths as GitHub.
			Codeberg => DefaultPkgUrls {
				paths: GITHUB_RELEASE_PATHS,
				filenames: &[FULL_FILENAMES, NOVERSION_FILENAMES],
				suffix: "",
			},
			Unknown => return None,
		})
	}

	pub fn get_default_pkg_url_template(self) -> Option<impl Iterator<Item = Template<'static>> + Clone + 'static> {
		let DefaultPkgUrls {
			paths,
			filenames,
			suffix,
		} = self.default_pkg_urls()?;
		Some(apply_filenames_to_paths(paths, filenames, suffix))
	}
}

fn apply_filenames_to_paths(
	paths: &'static [Template<'static>],
	filenames: &'static [&'static [Template<'static>]],
	suffix: &'static str,
) -> impl Iterator<Item = Template<'static>> + Clone + 'static {
	filenames
		.iter()
		.flat_map(|fs| fs.iter())
		.cartesian_product(paths.iter())
		.map(move |(filename, path)| {
			let mut template = path.clone() + filename;
			template += Item::Text(suffix);
			template
		})
}

#[cfg(test)]
mod crabx_tests {
	use super::*;

	#[test]
	fn crabx_tag() {
		let values = std::collections::HashMap::from([
			("repo", "https://github.com/moq-dev/moq"),
			("name", "moq-cli"),
			("version", "0.14.0"),
			("target", "x86_64-unknown-linux-gnu"),
			("archive-suffix", ".tar.gz"),
		]);
		let urls: Vec<_> = RepositoryHost::GitHub
			.get_default_pkg_url_template()
			.unwrap()
			.filter(|template| !template.has_key("subcrate"))
			.map(|template| template.render(&values).unwrap())
			.collect();
		for tag in ["0.14.0", "v0.14.0", "moq-cli-v0.14.0", "moq-cli-0.14.0"] {
			assert!(urls.contains(&format!(
				"https://github.com/moq-dev/moq/releases/download/{tag}/moq-cli-x86_64-unknown-linux-gnu-v0.14.0.tar.gz"
			)));
		}
	}
}
