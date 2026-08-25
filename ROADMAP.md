# 🗺️ Hoja de Ruta del Proyecto Ramati-Q

## ✅ FASE 1.0 - Núcleo Fundacional [COMPLETADA]
- Lenguaje Ramati-Q (sintaxis base).
- Compilador (Lexer, Parser, AST).
- Motor de Ejecución (Runtime).

## ✅ FASE 1.2 - Conciencia del Entorno [COMPLETADA]
- Memoria Contextual y Persistencia de Sesión.
- Generación de Telemetría (`.qbin`).
- Integración de Sensores y Actuadores (simulados).

## ✅ FASE 1.5 - Piloto Autónomo [CERTIFICADA]
- Navegación orientada a objetivos.
- Evasión de obstáculos y mínimos locales.
- Memoria espacial (mapa de celdas).
- Consolidación y modularización completa de la arquitectura.

## ➡️ FASE 2.0 - Sistema Cognitivo de Planificación [EN PLANIFICACIÓN]
- **Planificador de Rutas:** Algoritmos como A* para encontrar el camino óptimo.
- **Mapa Interno:** Construcción y gestión de un mapa del mundo a partir de la memoria espacial.
- **Abstracción de Hardware (HAL):** Implementación del `trait Hardware` para independizar el Kernel del simulador.
- **Integración Física:** Pruebas iniciales con hardware real (sensores, motores).
- **Gestión de Prioridades:** Sistema para manejar múltiples objetivos y tareas.