# Roadmap

## Fase 1 — Núcleo

- [x] Crear crate Rust.
- [x] Implementar matching por prefijo y contains.
- [x] Implementar estado de selección.
- [x] Crear el primer adaptador Zsh/ZLE.
- [x] Mostrar y navegar sugerencias desde Zsh.
- [ ] Añadir deduplicación configurable.
- [ ] Añadir carga/escritura atómica del historial.

## Fase 2 — macOS y Zsh

- [x] Definir adaptador ZLE.
- [x] Mostrar sugerencias debajo del buffer.
- [x] Navegar y aceptar sugerencias.
- [ ] Aceptar, cancelar y redimensionar correctamente.
- [ ] Probar en Terminal.app, iTerm2 y terminales compatibles.

## Fase 3 — Bash

- [ ] Extraer la integración Bash del proyecto anterior.
- [ ] Probar Bash 3.2 de macOS.
- [ ] Probar Bash moderno en Linux.

## Fase 4 — Distribución multiplataforma

- [ ] CI para macOS, Linux y Windows.
- [ ] Binarios para Intel y ARM.
- [ ] Instalación y actualización seguras.
- [ ] Documentación de configuración.
