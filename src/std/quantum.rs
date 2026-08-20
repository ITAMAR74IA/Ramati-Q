//! Módulo Cuántico - Ramati-Q std::quantum

pub struct QuantumState {
    pub probability: f32,
}

impl QuantumState {
    pub fn new(prob: f32) -> Self {
        Self {
            probability: prob.clamp(0.0, 1.0),
        }
    }

    pub fn superpose(&self, other: &Self) -> f32 {
        (self.probability + other.probability) / 2.0
    }
}
