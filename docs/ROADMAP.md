# Roadmap

## Fase 1 — Núcleo

- [x] Crear crate Rust.
- [x] Implementar matching por prefijo y contains.
- [x] Implementar estado de selección.
- [x] Crear el primer adaptador Zsh/ZLE.
- [x] Mostrar y navegar sugerencias desde Zsh.
- [x] Automatizar la prueba Zsh con un pseudo-terminal.
- [x] Deduplicar comandos repetidos y aplicar el límite después de deduplicar.
- [x] Separar la vista de predicciones de la navegación del historial nativo.
- [ ] Añadir carga/escritura atómica del historial.

## Fase 2 — macOS y Zsh

- [x] Definir adaptador ZLE.
- [x] Mostrar sugerencias debajo del buffer.
- [x] Navegar y aceptar sugerencias.
- [x] Ocultar predicciones con Escape y alternarlas con F2 en Zsh.
- [ ] Redimensionar correctamente.
- [ ] Probar en Terminal.app, iTerm2 y terminales compatibles.

## Fase 3 — Bash

- [x] Implementar el adaptador Bash independiente.
- [x] Añadir pruebas PTY para Bash.
- [x] Documentar la incompatibilidad del Bash 3.2 incluido con macOS.
- [x] Probar Bash 4+ de macOS mediante Homebrew.
- [x] Probar Bash moderno en Linux mediante CI.

## Fase 4 — Distribución multiplataforma

- [x] CI para macOS y Linux.
- [x] Binarios para Intel y ARM64.
- [x] Instalación verificada mediante checksum SHA-256.
- [x] Documentación de instalación y configuración.

Windows queda fuera del objetivo inmediato porque PSReadLine ya cubre esa
experiencia. El núcleo seguirá preparado para una futura integración nativa.
