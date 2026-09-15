# rRsReadLine

Rust-based shell history suggestions with a shell-independent core.

The project separates the suggestion engine from shell-specific integrations.
Spanish documentation is available in [`README.es.md`](README.es.md).

## Status

The core engine and the Zsh/ZLE and Bash/Readline integrations are functional.
The Unix integrations are validated on macOS and Linux through local and CI
tests. Windows remains intentionally deferred because PSReadLine already
provides this experience.

## Compatibility

| Component | Status |
| --- | --- |
| Rust suggestion engine | Portable by design |
| macOS + Zsh | Validated |
| Linux + Zsh | Validated in CI |
| Bash 4+ | Validated on macOS and Linux |
| Fish | Planned |
| PowerShell/Windows | PSReadLine is the existing recommended alternative |

## Goals

- Suggest commands from shell history while typing.
- Navigate suggestions with the arrow keys.
- Accept or cancel a suggestion without losing the current buffer.
- Keep the core independent from any particular shell or operating system.
- Provide separate adapters for Zsh, Bash, Fish, and PowerShell.

## Windows scope

rRsReadLine focuses on macOS and Linux. Windows already has a mature shell
history and prediction solution in PSReadLine, so a native rRsReadLine Windows
adapter is intentionally deferred. The core remains portable so a future
Windows integration can be added if a native implementation becomes useful.

## Installation

### Prebuilt binary

The installer downloads the release binary for macOS or Linux, verifies its
SHA-256 checksum, and installs it into `~/.local/bin` without requiring Rust:

```sh
curl --fail --silent --show-error --location \
  https://raw.githubusercontent.com/luisgamas/rrsreadline/main/scripts/install.sh \
  | sh
```

Set `RRSREADLINE_VERSION=vX.Y.Z` to install a specific release or
`RRSREADLINE_INSTALL_DIR=/custom/path` to choose another destination. The
installer does not modify shell startup files.

### Build from source

Developers with Rust can install the latest source directly:

```sh
cargo install --git https://github.com/luisgamas/rrsreadline --locked
```

## Try it with Zsh

Build the binary and evaluate the integration in the current Zsh session:

```sh
cargo build --release
eval "$(./target/release/rrsreadline init zsh)"
```

To enable it permanently, add the following to `~/.zshrc` using the absolute
path to the binary:

```sh
eval "$(/absolute/path/to/rrsreadline/target/release/rrsreadline init zsh)"
```

Use Up/Down to navigate the list, Tab or Enter to accept a selected history
suggestion, and Escape to clear the list. When no history suggestion is
selected, Tab keeps Zsh's native command, path, and file completion.

Optional configuration is read from
`~/.config/rrsreadline/config.toml` (for both Zsh and Bash):

```toml
matching = "prefix"
max_suggestions = 10
case_sensitive = false
history_file = "~/.zsh_history"
```

The default is 10 suggestions. Set `max_suggestions` to change the limit.
After changing this value, re-evaluate `rrsreadline init bash` in Bash.

## Bash

The Bash adapter requires Bash 4 or newer because it uses writable
`READLINE_LINE` and `READLINE_POINT` variables. The Bash 3.2 shipped with
macOS is too old for full integration. Install a current Bash with Homebrew:

```sh
brew install bash
```

Then start that Bash and evaluate:

```sh
eval "$(rrsreadline init bash)"
```

Use Up/Down to navigate suggestions and Enter to submit the selected command.
Tab remains Bash's native command, path, and file completion key. Enter accepts
the selected history suggestion.

## Development

```sh
cargo test
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

On Unix systems, `cargo test` runs Bash and Zsh pseudo-terminal integration
tests. Bash tests require Bash 4+ and Zsh tests require `zsh` on `PATH`.
The same checks run for macOS and Linux in GitHub Actions.

The architecture and implementation roadmap are documented in
[`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) and
[`docs/ROADMAP.md`](docs/ROADMAP.md).

## License

MIT. See [`LICENSE`](LICENSE).
