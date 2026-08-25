// =====================================================================
// Módulo de Entorno de Ejecución - Ramati-Q v1.5
// =====================================================================
// Este módulo define el entorno de ejecución (la tabla de símbolos)
// y los tipos de datos que el lenguaje puede manejar.

use std::collections::HashMap;
use std::fmt;

// 1. Tipos de datos que Ramati-Q puede manejar (Fase 0.7)
#[derive(Debug, Clone, PartialEq)]
pub enum ValorRamati {
    Entero(i64),
    Cadena(String),
    Espectro(Vec<f64>),
    Nulo,
}

// Implementamos el trait `Display` para un formato de impresión limpio.
impl fmt::Display for ValorRamati {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValorRamati::Entero(val) => write!(f, "{}", val),
            ValorRamati::Cadena(val) => write!(f, "{}", val),
            ValorRamati::Espectro(vals) => {
                let formatted_vals: Vec<String> = vals.iter().map(|v| format!("{:.4}", v)).collect();
                write!(f, "[{}]", formatted_vals.join(", "))
            }
            ValorRamati::Nulo => write!(f, "nulo"),
        }
    }
}

// 2. Entorno de Ejecución con soporte para scopes (Fase 0.8)
#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, ValorRamati>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn new_enclosed(enclosing: Environment) -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: Some(Box::new(enclosing)),
        }
    }

    pub fn get(&self, name: &str) -> Option<ValorRamati> {
        if let Some(value) = self.values.get(name) {
            Some(value.clone())
        } else if let Some(parent) = &self.enclosing {
            parent.get(name)
        } else {
            None
        }
    }

    pub fn define(&mut self, name: String, value: ValorRamati) {
        self.values.insert(name, value);
    }
}