// Programa de prueba para la Fase 2.0: Planificación Cognitiva
use ramati_q::std::planning::CognitivePlanner;

fn main() {
    println!("--- Iniciando Pruebas de Planificación Cognitiva (Fase 2.0) ---");

    // Definimos una estrategia de navegación con un umbral mínimo de certeza del 70% (0.7)
    let planificador = CognitivePlanner::new("Navegación Segura", 0.7);

    // Opciones posibles que el sistema evalúa con su certeza difusa
    let opciones = [
        ("Ruta Alfa (Marginal)", 0.65),
        ("Ruta Beta (Óptima)", 0.88), 
        ("Ruta Gamma (Inestable)", 0.40),
    ];

    match planificador.evaluate_plan(&opciones) {
        Ok(mensaje) => println!("Decisión Cognitiva -> {}", mensaje),
        Err(error) => println!("Error de Planificación -> {}", error),
    }

    println!("--- Pruebas de Planificación Concluidas ---");
}