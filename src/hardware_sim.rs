// =====================================================================
// Módulo de Simulación de Robótica Virtual - Ramati-Q v1.5
// =====================================================================
use std::collections::HashSet;

use crate::hal::Hardware;
use crate::entorno::ValorRamati;
use crate::mapa_cognitivo::{MapaCognitivo, EstadoCelda};

pub struct ObstaculoCircular {
    pub x: f64,
    pub y: f64,
    pub radio: f64,
}

pub struct RobotVirtual {
    pub x: f64,
    pub y: f64,
    pub theta: f64,             // Orientacion en radianes [-PI, PI]
    pub velocidad_lineal: f64,   // Controlado por mover_adelante/mover_atras
    pub velocidad_angular: f64,  // Controlado por girar_izq/girar_der
    pub colisionado: bool,
    pub objetivo_alcanzado: bool,
}

pub struct SimuladorRobotico2D {
    pub robot: RobotVirtual,
    pub obstaculos: Vec<ObstaculoCircular>,
    pub limite_mundo: f64,      // Dimensiones de la caja virtual (ej. 100.0)
    pub objetivo: Option<(f64, f64)>, // Coordenadas (x, y) del objetivo
    pub mapa_cognitivo: MapaCognitivo,
}

impl SimuladorRobotico2D {
    // Inicializacion del plano de simulacion con obstaculos fijos
    pub fn new() -> Self {
        let obstaculos = vec![
            ObstaculoCircular { x: 50.0, y: 50.0, radio: 8.0 },
            ObstaculoCircular { x: 30.0, y: 70.0, radio: 6.0 },
            ObstaculoCircular { x: 70.0, y: 25.0, radio: 10.0 },
        ];

        let limite_mundo = 100.0;
        let tamano_celda = 5.0;

        SimuladorRobotico2D {
            robot: RobotVirtual {
                x: 20.0,
                y: 20.0,
                theta: 0.0, // Mirando hacia el eje X positivo
                velocidad_lineal: 0.0,
                velocidad_angular: 0.0,
                colisionado: false,
                objetivo_alcanzado: false,
            },
            obstaculos,
            limite_mundo: 100.0,
            objetivo: None,
            mapa_cognitivo: MapaCognitivo::new(limite_mundo, limite_mundo, tamano_celda),
        }
    }

    // Aplica las leyes cinematicas en un paso de tiempo (delta_t)
    pub fn actualizar_fisica(&mut self, delta_t: f64) {
        if self.robot.colisionado {
            return; // Parada de emergencia fisica activa
        }

        // Capturar la posición anterior en el grid antes de mover el robot
        let old_grid_coords = self.mapa_cognitivo.coords_mundo_a_grid(self.robot.x, self.robot.y);

        // 1. Actualizar orientacion del chasis
        self.robot.theta += self.robot.velocidad_angular * delta_t;
        // Correcta normalización del ángulo para mantenerlo en el rango [-PI, PI]
        self.robot.theta = (self.robot.theta + std::f64::consts::PI).rem_euclid(2.0 * std::f64::consts::PI) - std::f64::consts::PI;

        // 2. Actualizar coordenadas espaciales (Cinematica diferencial)
        let dx = self.robot.velocidad_lineal * self.robot.theta.cos() * delta_t;
        let dy = self.robot.velocidad_lineal * self.robot.theta.sin() * delta_t;

        self.robot.x += dx;
        self.robot.y += dy;

        // 2.5. Actualizar mapa cognitivo
        if let Some((old_x, old_y)) = old_grid_coords {
            self.mapa_cognitivo.actualizar_celda(old_x, old_y, EstadoCelda::Visitado);
        }
        if let Some((new_x, new_y)) = self.mapa_cognitivo.coords_mundo_a_grid(self.robot.x, self.robot.y) {
            self.mapa_cognitivo.actualizar_celda(new_x, new_y, EstadoCelda::Robot);
        }

        // 2.8. Comprobar si se ha alcanzado el objetivo
        if let Some((tx, ty)) = self.objetivo {
            let dist_objetivo = ((self.robot.x - tx).powi(2) + (self.robot.y - ty).powi(2)).sqrt();
            if dist_objetivo < self.mapa_cognitivo.tamano_celda { // Si está dentro de una celda del objetivo
                self.robot.objetivo_alcanzado = true;
                self.robot.velocidad_lineal = 0.0;
                self.robot.velocidad_angular = 0.0;
                println!("[PLANIFICADOR] 🎯 ¡OBJETIVO ALCANZADO EN ({:.2}, {:.2})!", self.robot.x, self.robot.y);
            }
        }

        // 3. Evaluar colisiones contra las paredes del limite
        if self.robot.x < 0.0 || self.robot.x > self.limite_mundo ||
           self.robot.y < 0.0 || self.robot.y > self.limite_mundo {
            self.robot.colisionado = true;
            println!("[SIMULADOR] 💥 COLISION CRITICA DETECTADA CONTRA PARED LIMITROFE");
        }

        // 4. Evaluar colisiones contra obstaculos circulares
        for obstaculo in &self.obstaculos {
            let dist = ((self.robot.x - obstaculo.x).powi(2) + (self.robot.y - obstaculo.y).powi(2)).sqrt();
            if dist <= obstaculo.radio {
                self.robot.colisionado = true;
                println!("[SIMULADOR] 💥 COLISION DETECTADA CONTRA OBSTACULO EN ({}, {})", obstaculo.x, obstaculo.y);
            }
        }
    }

    // Simula raycasting de LiDAR angular
    pub fn calcular_distancia_sensor(&self, offset_angular: f64) -> f64 {
        let angulo_sensor = self.robot.theta + offset_angular;
        let mut min_distancia = 35.0; // Rango de lectura maxima del sensor

        for obs in &self.obstaculos {
            let dx = obs.x - self.robot.x;
            let dy = obs.y - self.robot.y;
            
            let cos_s = angulo_sensor.cos();
            let sin_s = angulo_sensor.sin();
            let proyeccion = dx * cos_s + dy * sin_s;

            if proyeccion > 0.0 {
                let dist_perpendicular_sq = dx.powi(2) + dy.powi(2) - proyeccion.powi(2);
                if dist_perpendicular_sq >= 0.0 {
                    let dist_perpendicular = dist_perpendicular_sq.sqrt();
                    if dist_perpendicular < obs.radio {
                        let interseccion = proyeccion - (obs.radio.powi(2) - dist_perpendicular_sq).sqrt();
                        if interseccion < min_distancia {
                            min_distancia = interseccion;
                        }
                    }
                }
            }
        }

        // Retorna lectura normalizada en espectro [0.0, 1.0]
        (min_distancia / 35.0).max(0.0).min(1.0)
    }
}

impl Hardware for SimuladorRobotico2D {
    fn capturar(&self, canal: &str) -> ValorRamati {
        let valor_sensor = match canal {
            "sensor_frontal" => self.calcular_distancia_sensor(0.0),
            "sensor_izquierdo" => self.calcular_distancia_sensor(std::f64::consts::FRAC_PI_4),
            "sensor_derecho" => self.calcular_distancia_sensor(-std::f64::consts::FRAC_PI_4),
            "colision" => if self.robot.colisionado { 1.0 } else { 0.0 },
            "distancia_objetivo" => {
                if let Some((tx, ty)) = self.objetivo {
                    ((tx - self.robot.x).powi(2) + (ty - self.robot.y).powi(2)).sqrt()
                } else { 0.0 }
            },
            "angulo_relativo_objetivo" => {
                if let Some((tx, ty)) = self.objetivo {
                    let angulo_hacia_objetivo = (ty - self.robot.y).atan2(tx - self.robot.x);
                    let angulo_relativo = angulo_hacia_objetivo - self.robot.theta;
                    angulo_relativo.atan2(angulo_relativo.cos())
                } else { 0.0 }
            },
            "zona_explorada" => {
                if let Some((grid_x, grid_y)) = self.mapa_cognitivo.coords_mundo_a_grid(self.robot.x, self.robot.y) {
                    if self.mapa_cognitivo.grid[grid_x][grid_y].estado == EstadoCelda::Visitado {
                        1.0
                    } else { 0.0 }
                } else { 0.0 }
            },
            _ => 0.0, // Sensor desconocido devuelve 0.0
        };
        ValorRamati::Espectro(vec![valor_sensor])
    }

    fn accionar(&mut self, actuador: &str, valor: f64) {
        println!("[ROBOT VIRTUAL] ⚡ Accionando '{}' con valor {:.2}", actuador, valor);
        match actuador {
            "avance_lineal" => self.robot.velocidad_lineal = valor * 5.0,
            "giro_derecha" => self.robot.velocidad_angular = -valor * 1.5,
            "giro_izquierda" => self.robot.velocidad_angular = valor * 1.5,
            _ => {} // Actuador desconocido no hace nada.
        }
    }

    fn establecer_objetivo(&mut self, x: f64, y: f64) {
        self.objetivo = Some((x, y));
        if let Some((grid_x, grid_y)) = self.mapa_cognitivo.coords_mundo_a_grid(x, y) {
            self.mapa_cognitivo.actualizar_celda(grid_x, grid_y, EstadoCelda::Objetivo);
        }
        println!("[PLANIFICADOR] ✅ Nuevo objetivo establecido en ({:.2}, {:.2})", x, y);
    }

    fn paso_fisico(&mut self, delta_t: f64) {
        self.actualizar_fisica(delta_t);
    }

    fn obtener_diagnostico(&self) -> String {
        let mut diag = String::new();
        diag.push_str("\n=== DIAGNÓSTICO ROBÓTICO (Ciclo) ===\n");
        diag.push_str(&format!("  Posición: (x: {:.2}, y: {:.2})\n", self.robot.x, self.robot.y));
        diag.push_str(&format!("  Orientación: {:.2} rad ({:.1} deg)\n", self.robot.theta, self.robot.theta.to_degrees()));
        if let Some((tx, ty)) = self.objetivo {
            diag.push_str(&format!("  Objetivo: (x: {:.2}, y: {:.2})\n", tx, ty));
        }
        diag.push_str(&format!("  Celdas de Mapa Cognitivo: {}\n", self.mapa_cognitivo.ancho * self.mapa_cognitivo.alto));
        diag.push_str(&format!("  Estado de Colisión: {}\n", self.robot.colisionado));
        diag.push_str("======================================\n");
        diag
    }

    fn objetivo_alcanzado(&self) -> bool {
        self.robot.objetivo_alcanzado
    }
}