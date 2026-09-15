#!/bin/sh

set -eu

repository="${RRSREADLINE_REPOSITORY:-luisgamas/rrsreadline}"
version="${RRSREADLINE_VERSION:-latest}"
home_dir="${HOME:-}"
install_dir="${RRSREADLINE_INSTALL_DIR:-${home_dir}/.local/bin}"

if [ -z "$home_dir" ] && [ -z "${RRSREADLINE_INSTALL_DIR:-}" ]; then
    printf '%s\n' 'rrsreadline: HOME is not set; use RRSREADLINE_INSTALL_DIR.' >&2
    exit 1
fi

if ! command -v curl >/dev/null 2>&1; then
    printf '%s\n' 'rrsreadline: curl is required.' >&2
    exit 1
fi

if ! command -v tar >/dev/null 2>&1; then
    printf '%s\n' 'rrsreadline: tar is required.' >&2
    exit 1
fi

os=$(uname -s)
arch=$(uname -m)

case "$os:$arch" in
    Darwin:x86_64)
        target="x86_64-apple-darwin"
        ;;
    Darwin:arm64 | Darwin:aarch64)
        target="aarch64-apple-darwin"
        ;;
    Linux:x86_64 | Linux:amd64)
        target="x86_64-unknown-linux-gnu"
        ;;
    Linux:aarch64 | Linux:arm64)
        target="aarch64-unknown-linux-gnu"
        ;;
    *)
        printf 'rrsreadline: no prebuilt binary for %s/%s.\n' "$os" "$arch" >&2
        printf '%s\n' 'Install Rust and use: cargo install --git https://github.com/luisgamas/rrsreadline' >&2
        exit 1
        ;;
esac

if [ "$version" = latest ]; then
    download_base="https://github.com/$repository/releases/latest/download"
else
    download_base="https://github.com/$repository/releases/download/$version"
fi

archive="rrsreadline-$target.tar.gz"
temporary_dir=$(mktemp -d "${TMPDIR:-/tmp}/rrsreadline-install.XXXXXX")
trap 'rm -rf "$temporary_dir"' EXIT HUP INT TERM

printf 'Downloading rrsreadline for %s...\n' "$target"
curl --fail --silent --show-error --location \
    "$download_base/$archive" \
    --output "$temporary_dir/$archive"
curl --fail --silent --show-error --location \
    "$download_base/rrsreadline-checksums.txt" \
    --output "$temporary_dir/rrsreadline-checksums.txt"

if command -v sha256sum >/dev/null 2>&1; then
    actual_checksum=$(sha256sum "$temporary_dir/$archive" | awk '{print $1}')
elif command -v shasum >/dev/null 2>&1; then
    actual_checksum=$(shasum -a 256 "$temporary_dir/$archive" | awk '{print $1}')
else
    printf '%s\n' 'rrsreadline: sha256sum or shasum is required.' >&2
    exit 1
fi

expected_checksum=$(awk -v archive="$archive" '$2 == archive { print $1; exit }' \
    "$temporary_dir/rrsreadline-checksums.txt")
if [ -z "$expected_checksum" ] || [ "$actual_checksum" != "$expected_checksum" ]; then
    printf '%s\n' 'rrsreadline: checksum verification failed.' >&2
    exit 1
fi

tar -xzf "$temporary_dir/$archive" -C "$temporary_dir"
if [ ! -f "$temporary_dir/rrsreadline" ]; then
    printf '%s\n' 'rrsreadline: downloaded archive did not contain the binary.' >&2
    exit 1
fi

mkdir -p "$install_dir"
install -m 0755 "$temporary_dir/rrsreadline" "$install_dir/rrsreadline"

printf 'Installed rrsreadline to %s/rrsreadline\n' "$install_dir"
case ":${PATH:-}:" in
    *":$install_dir:"*) ;;
    *) printf 'Add %s to PATH if it is not already included.\n' "$install_dir" ;;
esac
printf '%s\n' 'Then enable a shell integration with: rrsreadline init zsh or rrsreadline init bash'
