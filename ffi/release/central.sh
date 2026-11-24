#!/usr/bin/env bash
# Signs the Maven repository the builder wrote, one version of one artifact, and uploads it to
# the Central Portal: `publish` publishes it, and `validate` waits for the Portal's validation,
# then drops the deployment, publishing nothing.
#
#   central.sh publish|validate <repository>
#
# Takes the Portal user token as USERNAME and PASSWORD, and the ASCII-armored private key and
# its passphrase as SIGNING_KEY and SIGNING_PASSWORD.
set -euo pipefail

mode=${1:-}
repo=${2:-}
if [[ $# -ne 2 || ("$mode" != publish && "$mode" != validate) ]]; then
    echo "usage: central.sh publish|validate <repository>" >&2
    exit 2
fi
for var in USERNAME PASSWORD SIGNING_KEY; do
    if [[ -z "${!var:-}" ]]; then
        echo "::error::$var is unset" >&2
        exit 1
    fi
done
tmp=$(mktemp -d)
cd "$repo"
mapfile -t poms < <(find . -name '*.pom')
if [[ ${#poms[@]} -ne 1 ]]; then
    echo "::error::$repo holds ${#poms[@]} POMs, not one version of one artifact" >&2
    exit 1
fi
dir=$(dirname "${poms[0]#./}")
version=$(basename "$dir")
artifact=$(basename "$(dirname "$dir")")
group=$(dirname "$(dirname "$dir")" | tr / .)
api=https://central.sonatype.com/api/v1/publisher
auth="Authorization: Bearer $(printf '%s:%s' "$USERNAME" "$PASSWORD" | base64 -w0)"
mapfile -t files < <(find "$dir" -type f ! -name '*.md5' ! -name '*.sha1' ! -name '*.asc' | sort)

# A rerun of a release uploads nothing once Central has published the version, and fails on a
# file it serves with other bytes. A validation uploads regardless: the Portal refuses a
# published version, which is its answer.
if [[ "$mode" == publish ]]; then
    published=$(curl -sSf -H "$auth" "$api/published?namespace=$group&name=$artifact&version=$version")
    published=$(jq -r '.published | if type == "boolean" then . else error("no published flag") end' <<<"$published")
    if [[ "$published" == true ]]; then
        for file in "${files[@]}"; do
            if ! curl -sSfLo "$tmp/hosted" "https://repo1.maven.org/maven2/$file"; then
                echo "::error::Central published $group:$artifact:$version but does not serve $file yet: rerun once it does"
                exit 1
            fi
            cmp "$tmp/hosted" "$file" || {
                echo "::error::Central hosts another $file"
                exit 1
            }
        done
        echo "Central already hosts $group:$artifact:$version"
        exit 0
    fi
fi

export GNUPGHOME="$tmp/gnupg"
mkdir -m 700 "$GNUPGHOME"
printf '%s\n' "$SIGNING_KEY" | gpg --batch --quiet --import
for file in "${files[@]}"; do
    printf '%s' "${SIGNING_PASSWORD:-}" | gpg --batch --yes --pinentry-mode loopback --passphrase-fd 0 --armor --detach-sign "$file"
done
bundle="$tmp/bundle.zip"
zip -qr "$bundle" .

if [[ "$mode" == publish ]]; then
    type=AUTOMATIC done=PUBLISHED
else
    type=USER_MANAGED done=VALIDATED
fi
id=$(curl -sSf -H "$auth" -F "bundle=@$bundle" "$api/upload?publishingType=$type&name=$group:$artifact:$version")
echo "Deployment $id"
if [[ "$mode" == validate ]]; then
    # A validated or failed deployment is dropped, so no one can publish it from the Portal; a
    # drop that fails fails the run.
    # shellcheck disable=SC2329 # the trap below calls it
    drop() {
        local code=$?
        if ! curl -sSf -X DELETE -H "$auth" "$api/deployment/$id"; then
            echo "::error::could not drop deployment $id: drop it on the Portal before it can be published"
            exit 1
        fi
        echo "Dropped deployment $id"
        exit "$code"
    }
    trap drop EXIT
fi
# Central validates the bundle, then publishes it, which takes minutes.
for _ in $(seq 160); do
    status=$(curl -sSf -X POST -H "$auth" "$api/status?id=$id")
    state=$(jq -r .deploymentState <<<"$status")
    case $state in
        "$done")
            echo "Central $state $group:$artifact:$version"
            exit 0
            ;;
        FAILED)
            echo "::error::Central refused $group:$artifact:$version: $(jq -c .errors <<<"$status")"
            exit 1
            ;;
    esac
    sleep 30
done
echo "::error::Central did not reach $done for $group:$artifact:$version within 80 minutes: deployment $id"
exit 1
