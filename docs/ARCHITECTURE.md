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

La primera integración fue Zsh en macOS, usando ZLE. La siguiente es Bash 4+,
usando `bind -x` y Readline. El Bash 3.2 distribuido con macOS no permite la
actualización efectiva de `READLINE_LINE` desde un binding, por lo que no puede
ofrecer la integración completa.
Cada integración debe vivir en su propio módulo y tener pruebas específicas,
porque los shells tienen modelos distintos para editar y redibujar la línea.

Windows no es un objetivo inmediato: PSReadLine ya ofrece una solución nativa
de historial y predicción. El núcleo sigue sin dependencias de Windows para
que una futura integración propia siga siendo posible.

## Decisiones pendientes

- Protocolo entre el adaptador y el binario.
- Renderizado inline frente a `POSTDISPLAY`/equivalentes del shell.
- Persistencia y formato de cada historial.
- Búsqueda fuzzy opcional.
- Distribución por Homebrew y Linux.
