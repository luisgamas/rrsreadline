# rRsReadLine

Rust-based shell history suggestions with a shell-independent core.

The project separates the suggestion engine from shell-specific integrations.
Spanish documentation is available in [`README.es.md`](README.es.md).

## Status

The core engine is functional and the first experimental Zsh integration is
available. This release is not fully cross-platform yet: the core is designed
to be portable, but the currently working integration is Zsh/ZLE and has been
tested on macOS.

## Compatibility

| Component | Status |
| --- | --- |
| Rust suggestion engine | Portable by design |
| macOS + Zsh | First working integration |
| Linux + Zsh | Not validated yet |
| Bash | Planned |
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

Use Up/Down to navigate the list, Tab or Enter to accept the selected
suggestion, and Escape to clear the list.

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
Tab remains Bash's native completion key.

## Development

```sh
cargo test
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

On Unix systems, `cargo test` also runs a Zsh pseudo-terminal integration
test. It requires `zsh` to be available on `PATH`.

The architecture and implementation roadmap are documented in
[`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) and
[`docs/ROADMAP.md`](docs/ROADMAP.md).

## License

MIT. See [`LICENSE`](LICENSE).
