# rRsReadLine

Sugerencias de historial para shells, construido en Rust y con un núcleo
independiente del shell.

El proyecto separa el motor de sugerencias de las integraciones específicas de
cada shell. La documentación principal está disponible en
[`README.md`](README.md), en inglés.

## Estado

El núcleo funcional está listo y existe una primera integración experimental
con Zsh. Esta versión todavía no es completamente multiplataforma: el núcleo
está diseñado para ser portable, pero la integración disponible actualmente es
Zsh/ZLE y ha sido probada en macOS.

## Compatibilidad

| Componente | Estado |
| --- | --- |
| Motor de sugerencias Rust | Portable por diseño |
| macOS + Zsh | Primera integración funcional |
| Linux + Zsh | Todavía no validado |
| Bash | Planeado |
| Fish | Planeado |
| PowerShell/Windows | PSReadLine es la alternativa recomendada |

## Objetivos

- Sugerir comandos del historial mientras se escribe.
- Navegar por las sugerencias con las teclas de dirección.
- Aceptar o cancelar una sugerencia sin perder el buffer actual.
- Mantener el núcleo independiente del shell y del sistema operativo.
- Proporcionar adaptadores separados para Zsh, Bash, Fish y PowerShell.

## Alcance en Windows

rRsReadLine se enfocará en macOS y Linux. Windows ya cuenta con una solución
madura de historial y predicción mediante PSReadLine, por lo que el adaptador
nativo de rRsReadLine para Windows queda deliberadamente pospuesto. El núcleo
seguirá siendo portable por si más adelante resulta útil crear esa integración.

## Probarlo con Zsh

Compila el binario y evalúa la integración en la sesión actual de Zsh:

```sh
cargo build --release
eval "$(./target/release/rrsreadline init zsh)"
```

Para activarlo permanentemente, añade lo siguiente a `~/.zshrc`, usando la
ruta absoluta al binario:

```sh
eval "$(/ruta/absoluta/a/rrsreadline/target/release/rrsreadline init zsh)"
```

Usa Up/Down para navegar, Tab o Enter para aceptar la sugerencia seleccionada
y Escape para limpiar la lista.

La configuración opcional se lee desde
`~/.config/rrsreadline/config.toml` (para Zsh y Bash):

```toml
matching = "prefix"
max_suggestions = 10
case_sensitive = false
history_file = "~/.zsh_history"
```

El valor predeterminado es de 10 sugerencias. Cambia `max_suggestions` para
modificar el límite. Después de cambiarlo, vuelve a evaluar `rrsreadline init
bash` en Bash.

## Bash

El adaptador de Bash requiere Bash 4 o posterior porque utiliza las variables
modificables `READLINE_LINE` y `READLINE_POINT`. El Bash 3.2 incluido con
macOS es demasiado antiguo para una integración completa. Instala una versión
actual con Homebrew:

```sh
brew install bash
```

Después inicia ese Bash y evalúa:

```sh
eval "$(rrsreadline init bash)"
```

Usa Up/Down para navegar y Enter para ejecutar el comando seleccionado. Tab
conserva la función de completado nativa de Bash.

## Desarrollo

```sh
cargo test
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

En sistemas Unix, `cargo test` también ejecuta una prueba de integración de
Zsh dentro de un pseudo-terminal. Requiere que `zsh` esté disponible en
`PATH`.

La arquitectura y el roadmap están documentados en
[`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) y
[`docs/ROADMAP.md`](docs/ROADMAP.md).

## Licencia

MIT. Consulta [`LICENSE`](LICENSE).
