// Programa de prueba para la Librería Estándar de Ramati-Q v1.5

// Importamos los módulos de nuestra librería
use ramati_q::std::fuzzy::FuzzyEngine;
use ramati_q::std::quantum::QuantumState;
use ramati_q::std::spatial::Vector2D;
use ramati_q::std::io::BlackBox;

fn main() {
    println!("--- Iniciando Pruebas de Ramati-Q v1.5 ---");

    // 1. Prueba del Motor Difuso
    let fuzzy = FuzzyEngine::new();
    let certeza = fuzzy.evaluate(0.85);
    println!("Motor Difuso -> Certeza evaluada: {}", certeza);

    // 2. Prueba del Estado Cuántico
    let estado1 = QuantumState::new(0.6);
    let estado2 = QuantumState::new(0.4);
    let superposicion = estado1.superpose(&estado2);
    println!("Módulo Cuántico -> Superposición calculada: {}", superposicion);

    // 3. Prueba Espacial (Navegación 2D)
    let punto_a = Vector2D::new(0.0, 0.0);
    let punto_b = Vector2D::new(3.0, 4.0);
    let distancia = punto_a.distance_to(&punto_b);
    println!("Módulo Espacial -> Distancia al objetivo: {}", distancia);

    // 4. Prueba de I/O (Caja Negra)
    let caja_negra = BlackBox::new("ramati_log.qbin");
    println!("Módulo I/O -> Caja negra configurada en: {}", caja_negra.filepath);
    
    println!("--- Todas las pruebas ejecutadas con éxito ---");
}
