#!/bin/sh

set -eu

repository="${RRSREADLINE_REPOSITORY:-luisgamas/rrsreadline}"
version="${RRSREADLINE_VERSION:-latest}"
home_dir="${HOME:-}"
install_dir="${RRSREADLINE_INSTALL_DIR:-${home_dir}/.local/bin}"
shell_mode=auto
configure_shell=1

usage() {
    cat <<'EOF'
Usage: install.sh [--shell auto|zsh|bash|none] [--no-config]

Install the latest prebuilt rrsreadline binary and configure the selected
interactive shell. The shell configuration is idempotent and is not repeated
when an rrsreadline integration already exists.
EOF
}

while [ "$#" -gt 0 ]; do
    case "$1" in
        --shell)
            if [ "$#" -lt 2 ]; then
                printf '%s\n' 'rrsreadline: --shell requires auto, zsh, bash, or none.' >&2
                exit 2
            fi
            shell_mode="$2"
            shift 2
            ;;
        --no-config)
            configure_shell=0
            shift
            ;;
        --help|-h)
            usage
            exit 0
            ;;
        *)
            printf 'rrsreadline: unknown option: %s\n' "$1" >&2
            usage >&2
            exit 2
            ;;
    esac
done

if [ -z "$home_dir" ] && [ -z "${RRSREADLINE_INSTALL_DIR:-}" ]; then
    printf '%s\n' 'rrsreadline: HOME is not set; use RRSREADLINE_INSTALL_DIR.' >&2
    exit 1
fi

case "$shell_mode" in
    auto|zsh|bash|none) ;;
    *)
        printf 'rrsreadline: unsupported shell: %s\n' "$shell_mode" >&2
        exit 2
        ;;
esac

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

normalize_shell_name() {
    candidate=$1
    candidate=$(printf '%s' "$candidate" | sed 's|.*/||; s/^-//')
    case "$candidate" in
        zsh|bash) printf '%s\n' "$candidate" ;;
        *) printf '%s\n' '' ;;
    esac
}

detect_shell() {
    parent_shell=$(ps -p "$PPID" -o comm= 2>/dev/null || true)
    detected=$(normalize_shell_name "$parent_shell")
    if [ -n "$detected" ]; then
        printf '%s\n' "$detected"
        return
    fi

    login_shell=$(normalize_shell_name "${SHELL-}")
    printf '%s\n' "$login_shell"
}

shell_quote() {
    value=$1
    value=$(printf '%s' "$value" | sed "s/'/'\\\\''/g")
    printf "'%s'\n" "$value"
}

configure_shell() {
    selected_shell=$1
    binary_path="$install_dir/rrsreadline"
    quoted_binary=$(shell_quote "$binary_path")
    quoted_install_dir=$(shell_quote "$install_dir")

    case "$selected_shell" in
        zsh)
            config_file="$home_dir/.zshrc"
            ;;
        bash)
            bash_major=$(bash -c 'printf "%s" "${BASH_VERSINFO[0]}"' 2>/dev/null || printf '0')
            case "$bash_major" in
                0|1|2|3)
                    printf '%s\n' 'rrsreadline: Bash 4 or newer is required for shell integration.' >&2
                    printf '%s\n' 'The binary was installed, but the Bash startup file was not changed.' >&2
                    return
                    ;;
            esac
            if [ "$os" = Darwin ]; then
                config_file="$home_dir/.bash_profile"
            else
                config_file="$home_dir/.bashrc"
            fi
            ;;
        none)
            return
            ;;
    esac

    if [ -z "$home_dir" ]; then
        printf '%s\n' 'rrsreadline: HOME is required to configure a shell.' >&2
        return
    fi

    integration_pattern="init $selected_shell)"
    marker_pattern="# >>> rrsreadline initialize >>>"
    if [ -f "$config_file" ] && {
        grep -Fq "$integration_pattern" "$config_file" ||
        grep -Fq "$marker_pattern" "$config_file"
    }; then
        printf 'Shell integration already exists in %s; no duplicate was added.\n' "$config_file"
        return
    fi

    config_dir=$(dirname "$config_file")
    mkdir -p "$config_dir"
    temporary_config=$(mktemp "$config_file.rrsreadline.XXXXXX")
    if [ -f "$config_file" ]; then
        if [ ! -e "$config_file.rrsreadline.bak" ]; then
            cp -p "$config_file" "$config_file.rrsreadline.bak"
        fi
        cat "$config_file" > "$temporary_config"
    fi

    path_line=$(printf 'export PATH=%s:$PATH' "$quoted_install_dir")
    integration_line=$(printf 'eval "$(%s init %s)"' "$quoted_binary" "$selected_shell")
    printf '\n# >>> rrsreadline initialize >>>\n%s\n%s\n# <<< rrsreadline initialize <<<\n' \
        "$path_line" "$integration_line" >> "$temporary_config"

    if [ -f "$config_file" ]; then
        if [ "$os" = Darwin ]; then
            config_mode=$(stat -f '%Lp' "$config_file" 2>/dev/null || true)
        else
            config_mode=$(stat -c '%a' "$config_file" 2>/dev/null || true)
        fi
        if [ -n "$config_mode" ]; then
            chmod "$config_mode" "$temporary_config"
        fi
    fi
    mv "$temporary_config" "$config_file"
    printf 'Configured %s integration in %s.\n' "$selected_shell" "$config_file"
    printf 'Open a new terminal, or run: . %s\n' "$config_file"
}

if [ "$configure_shell" -eq 1 ]; then
    if [ "$shell_mode" = auto ]; then
        shell_mode=$(detect_shell)
    fi
    if [ -z "$shell_mode" ]; then
        printf '%s\n' 'Could not detect Zsh or Bash; the binary was installed without shell configuration.' >&2
        printf '%s\n' 'Re-run with --shell zsh or --shell bash.' >&2
    elif [ "$shell_mode" = none ]; then
        printf '%s\n' 'Shell configuration skipped (--shell none).'
    else
        configure_shell "$shell_mode"
    fi
else
    printf '%s\n' 'Shell configuration skipped (--no-config).'
fi
