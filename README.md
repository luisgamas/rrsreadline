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
| PowerShell/Windows | Planned |

## Goals

- Suggest commands from shell history while typing.
- Navigate suggestions with the arrow keys.
- Accept or cancel a suggestion without losing the current buffer.
- Keep the core independent from any particular shell or operating system.
- Provide separate adapters for Zsh, Bash, Fish, and PowerShell.

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
`~/.config/rrsreadline/config.toml`:

```toml
matching = "prefix"
max_suggestions = 8
case_sensitive = false
history_file = "~/.zsh_history"
```

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
