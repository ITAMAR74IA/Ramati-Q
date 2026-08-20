//! Módulo de Navegación y Control Espacial - Ramati-Q std::spatial

pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

impl Vector2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn distance_to(&self, target: &Self) -> f32 {
        ((target.x - self.x).powi(2) + (target.y - self.y).powi(2)).sqrt()
    }
}
