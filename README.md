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

The installer detects your parent shell, downloads the release binary for
macOS or Linux, verifies its SHA-256 checksum, installs it into
`~/.local/bin`, and configures the shell startup file without requiring Rust:

```sh
curl --fail --silent --show-error --location \
  https://raw.githubusercontent.com/luisgamas/rrsreadline/main/scripts/install.sh \
  | sh
```

Open a new terminal after installation. The installer adds an idempotent,
marked block to `~/.zshrc` for Zsh, `~/.bashrc` for Bash on Linux, or
`~/.bash_profile` for Bash on macOS. It creates a
`<startup-file>.rrsreadline.bak` backup before the first modification and never
adds a second integration block.

Use `--shell zsh` or `--shell bash` when automatic detection is unavailable.
Use `--no-config` to install only the binary. Set `RRSREADLINE_VERSION=vX.Y.Z`
to install a specific release or `RRSREADLINE_INSTALL_DIR=/custom/path` to
choose another destination:

```sh
curl --fail --silent --show-error --location \
  https://raw.githubusercontent.com/luisgamas/rrsreadline/main/scripts/install.sh \
  | sh -s -- --shell zsh
```

### Installation safety

The installer uses HTTPS, verifies the downloaded release archive against the
published SHA-256 checksums before installing it, writes only to the selected
binary directory and shell startup file, creates a backup before the first
startup-file change, and does not require `sudo`. The default command executes
the installer script from the repository's `main` branch; review or pin the
script and release in audited environments instead of using an unpinned
`curl | sh` command.

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

Use Up/Down to navigate the prediction list. Press Escape to hide predictions
and return Up/Down to native history navigation; press F2 to toggle the list
without changing the buffer. Tab keeps Zsh's native command, path, and file
completion. Enter accepts the currently selected line.

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
Press Escape to hide predictions and return Up/Down to native history
navigation. Tab remains Bash's native command,
path, and file completion key.

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
