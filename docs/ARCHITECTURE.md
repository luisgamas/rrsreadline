# Arquitectura inicial

El motor de rRsReadLine no debe conocer Bash, Zsh, PowerShell ni una ruta
concreta de historial. Esas responsabilidades pertenecen a adaptadores.

## Capas

1. `src/matching.rs`: transforma texto e historial en sugerencias.
2. `src/state.rs`: administra selección y eventos del editor.
3. `src/history.rs`: representa entradas, sin asumir el formato de un shell.
4. `src/config.rs`: configuración serializable y portable.
5. Adaptadores futuros: traducen eventos del shell al motor.

## Primera integración

La primera integración será Zsh en macOS, usando ZLE. Después se añadirá Bash.
Cada integración debe vivir en su propio módulo y tener pruebas específicas,
porque los shells tienen modelos distintos para editar y redibujar la línea.

## Decisiones pendientes

- Protocolo entre el adaptador y el binario.
- Renderizado inline frente a `POSTDISPLAY`/equivalentes del shell.
- Persistencia y formato de cada historial.
- Búsqueda fuzzy opcional.
- Distribución por Homebrew, Linux y Windows.
