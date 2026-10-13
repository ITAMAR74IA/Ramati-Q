//! Módulo de Lógica Difusa - Ramati-Q std::fuzzy

pub struct FuzzyEngine {
    pub active_rules: usize,
}

impl FuzzyEngine {
    pub fn new() -> Self {
        Self { active_rules: 0 }
    }

    /// Evalúa y limita el grado de certeza o pertenencia entre 0.0 y 1.0
    pub fn evaluate(&self, certainty: f32) -> f32 {
        certainty.clamp(0.0, 1.0)
    }

    /// Evalúa el espectro cognitivo del motor difuso en español
    pub fn evaluar_espectro(&self, valor: f32) -> &str {
        let cer = self.evaluate(valor);
        if cer < 0.3 {
            "Bajo / Precaución Estricta"
        } else if cer < 0.7 {
            "Medio / Espectro de Transición"
        } else {
            "Alto / Dominio Óptimo"
        }
    }
}