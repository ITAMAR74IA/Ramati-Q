// =====================================================================
// Módulo del Mapa Cognitivo - Ramati-Q v2.0
// =====================================================================
// Este módulo define la representación interna del mundo que el robot
// construye y utiliza para la planificación de rutas.

/// Estado de una celda en el mapa mental del robot.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EstadoCelda {
    Desconocido,
    Libre,
    Obstaculo,
    Visitado,
    Robot,
    Objetivo,
}

/// Representa una única celda en el grid del mapa.
#[derive(Clone, Copy, Debug)]
pub struct Celda {
    pub estado: EstadoCelda,
    // Futuro: costo_g, costo_h, costo_f para A*
}

impl Celda {
    pub fn new() -> Self {
        Celda { estado: EstadoCelda::Desconocido }
    }
}

/// Representa el mapa mental completo que el robot construye.
pub struct MapaCognitivo {
    pub grid: Vec<Vec<Celda>>,
    pub ancho: usize,
    pub alto: usize,
    pub tamano_celda: f64,
}

impl MapaCognitivo {
    pub fn new(ancho_mundo: f64, alto_mundo: f64, tamano_celda: f64) -> Self {
        let ancho = (ancho_mundo / tamano_celda).ceil() as usize;
        let alto = (alto_mundo / tamano_celda).ceil() as usize;
        let grid = vec![vec![Celda::new(); alto]; ancho];

        MapaCognitivo {
            grid,
            ancho,
            alto,
            tamano_celda,
        }
    }

    /// Convierte coordenadas del mundo a coordenadas del grid.
    pub fn coords_mundo_a_grid(&self, x: f64, y: f64) -> Option<(usize, usize)> {
        let grid_x = (x / self.tamano_celda) as usize;
        let grid_y = (y / self.tamano_celda) as usize;
        if grid_x < self.ancho && grid_y < self.alto {
            Some((grid_x, grid_y))
        } else {
            None
        }
    }

    /// Actualiza el estado de una celda.
    pub fn actualizar_celda(&mut self, x: usize, y: usize, nuevo_estado: EstadoCelda) {
        if x < self.ancho && y < self.alto {
            // No sobreescribir un obstáculo o el objetivo con un estado de 'visitado' o 'robot'
            if self.grid[x][y].estado != EstadoCelda::Obstaculo && self.grid[x][y].estado != EstadoCelda::Objetivo {
                 self.grid[x][y].estado = nuevo_estado;
            }
        }
    }
}