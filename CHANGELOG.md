# Historial de Cambios - Ramati-Q

## [1.5.0] - 2024-XX-XX

### `CERTIFICADO`
- Esta es la primera versión pública y certificada del Kernel Ramati-Q.

### Añadido
- **Piloto Autónomo:** Sistema completo de navegación orientada a objetivos.
- **Evasión Estratégica:** Lógica de "seguimiento de pared" para superar mínimos locales.
- **Memoria Espacial:** Capacidad del agente para recordar zonas visitadas.
- **HAL (Simulada):** Capa de abstracción de hardware con un robot virtual 2D.
- **Motor de Reglas:** Soporte para `regla` y `razonar` (encadenamiento hacia adelante).
- **Modularización Completa:** El código fuente del Kernel ha sido refactorizado en módulos de alta cohesión (`lexer`, `parser`, `semantic`, `runtime`).
- **Documentación Maestra:** Creación de `README.md`, `LICENSE`, `CHANGELOG.md` y `ROADMAP.md`.

### Cambiado
- **Optimización Semántica:** El analizador semántico ahora usa `HashSet` para una verificación de variables más rápida.