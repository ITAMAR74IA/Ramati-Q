//! Módulo de Lógica Difusa - Ramati-Q std::fuzzy

pub struct FuzzyEngine {
    pub active_rules: usize,
}

impl FuzzyEngine {
    pub fn new() -> Self {
        Self { active_rules: 0 }
    }

    pub fn evaluate(&self, certainty: f32) -> f32 {
        certainty.clamp(0.0, 1.0)
    }
}
