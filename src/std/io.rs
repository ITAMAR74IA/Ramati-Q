//! Módulo de Persistencia I/O - Ramati-Q std::io

pub struct BlackBox {
    pub filepath: String,
}

impl BlackBox {
    pub fn new(path: &str) -> Self {
        Self {
            filepath: path.to_string(),
        }
    }

    pub fn save_log(&self, data: &[u8]) -> bool {
        !data.is_empty()
    }
}
