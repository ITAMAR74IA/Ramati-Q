// Módulo de Planificación Cognitiva - Ramati-Q v1.5
// Diseñado para evaluar rutas, calcular certezas difusas y tomar decisiones adaptativas.

pub struct CognitivePlanner {
    pub strategy_name: String,
    pub threshold: f64,
}

impl CognitivePlanner {
    /// Inicializa un nuevo planificador cognitivo con un umbral de certeza por defecto
    pub fn new(strategy: &str, threshold: f64) -> Self {
        CognitivePlanner {
            strategy_name: strategy.to_string(),
            threshold,
        }
    }

    /// Evalúa una lista de opciones de acción basadas en un puntaje de certeza difusa (0.0 a 1.0)
    /// Retorna la mejor opción si supera el umbral establecido.
    pub fn evaluate_plan(&self, options: &[(&str, f64)]) -> Result<String, String> {
        let mut best_option = "";
        let mut highest_score = -1.0;

        for &(name, score) in options {
            if score > highest_score {
                highest_score = score;
                best_option = name;
            }
        }

        if highest_score >= self.threshold {
            Ok(format!(
                "Plan aprobado bajo estrategia '{}'. Mejor opción: '{}' con certeza de {:.2}",
                self.strategy_name, best_option, highest_score
            ))
        } else {
            Err(format!(
                "Rechazado: Ninguna opción alcanza el umbral mínimo de certeza ({:.2}).",
                self.threshold
            ))
        }
    }
}