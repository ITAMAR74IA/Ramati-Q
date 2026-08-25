use std::fs;
use serde::Deserialize;
use std::time::Instant;

// Módulos del Kernel Ramati-Q
mod hal;
mod mapa_cognitivo;
mod memoria_contextual; // Módulo de persistencia de sesión
mod hardware_sim;     // Módulo de simulación de robótica virtual
mod entorno;          // Módulo de entorno de ejecución y tipos de datos
mod ast;              // Módulo del Árbol de Sintaxis Abstracto (AST)
mod lexer;
mod parser;
mod semantic;
mod runtime;

use ast::*;
use lexer::ejecutar_lexer;
use parser::ejecutar_parser;
use semantic::verificar_semantica;
use runtime::ejecutar_runtime;

// =========================================================================
// 2. CAPAS DEL COMPILADOR RAMATI-Q
// =========================================================================

// =========================================================================
// 3. FLUJO PRINCIPAL Y TELEMETRÍA
// =========================================================================

#[derive(Deserialize, Debug)]
struct Configuracion {
    modo_reporte: String,
    version_kernel: String,
}

fn leer_configuracion() -> Configuracion {
    let contenido = fs::read_to_string("configuracion.rmq")
        .expect("¡No se encontró configuracion.rmq!");
    serde_json::from_str(&contenido).expect("Error en formato de configuracion.rmq")
}

fn main() {
    // --- SECUENCIA DE ARRANQUE VISUAL (Bootloader) ---
    println!("\n\n");
    println!("  ========================================================");
    println!("     ____    _    __  __    _  _____ ___        ___  ");
    println!("    |  _ \\  / \\  |  \\/  |  / \\|_   _|_ _|      / _ \\ ");
    println!("    | |_) |/ _ \\ | |\\/| | / _ \\ | |  | | _____| | | |");
    println!("    |  _ &lt;/ ___ \\| |  | |/ ___ \\| |  | | |____| |_| |");
    println!("    |_| \\_\\_/   \\_\\_|  |_/_/   \\_\\_| |___|     \\__\\_\\");
    println!("                                                      ");
    println!("                    Cuántico Difuso                   ");
    println!("  ========================================================");
    println!("  >>> INICIANDO RAMATI-Q KERNEL [FASE 1.5]...");
    println!("  ========================================================\n");
    
    let config = leer_configuracion();

    println!("[SISTEMA] Leyendo archivo de código 'main.rmq' local... ");
    let codigo_fuente = fs::read_to_string("main.rmq")
        .expect("❌ ERROR CRÍTICO: No se pudo encontrar el archivo 'main.rmq'.");

    let t_inicio = Instant::now();

   // 1. Capa 1: Lexer
    let t_l0 = Instant::now();
    let tokens = ejecutar_lexer(&codigo_fuente);
    let d_lexer = t_l0.elapsed().as_nanos();

    // 📸 --- BLOQUE TEMPORAL DE INSPECCIÓN VISUAL ---
    println!("\n📸 [INSPECTOR] Fichas léxicas generadas por el Lexer:");
    for token in &tokens {
        println!("  => {:?}", token);
    }
    println!("--------------------------------------------------\n");

    // 2. Capa 2: Parser
    let t_p0 = Instant::now();
    let ast = match ejecutar_parser(&tokens) {
        Ok(arbol) => arbol,
        Err(e) => { println!("Error en parser: {}", e); std::process::exit(1); }
    };
    let d_parser = t_p0.elapsed().as_nanos();

    // 3. Capa 3: Analizador Semántico
    let t_s0 = Instant::now();
    match verificar_semantica(&ast) {
        Ok(_) => println!("[VERIFICADOR] ✅ Análisis de pre-vuelo exitoso. Variables e integridad coherentes."),
        Err(e) => { eprintln!("{}", e); std::process::exit(1); }
    }
    let d_semantico = t_s0.elapsed().as_nanos();
    
    // 4. Capa 4: Runtime Cognitivo
    let t_r0 = Instant::now();
    
    // Creamos el entorno global y la tabla de funciones
    let mut environment = entorno::Environment::new();
    let mut funciones: std::collections::HashMap<String, &Nodo> = std::collections::HashMap::new();
    let mut reglas: std::collections::HashMap<String, &Nodo> = std::collections::HashMap::new();
    let mut simulador_robotico = hardware_sim::SimuladorRobotico2D::new();

    // Primera pasada: Registrar todas las funciones (Hoisting)
    // y reglas para la base de conocimiento.
    if let Nodo::Programa(nodos) = &ast {
        for nodo in nodos {
            match nodo {
                Nodo::DeclaracionFuncion { nombre, .. } => {
                    funciones.insert(nombre.clone(), nodo);
                }
                Nodo::DeclaracionRegla { nombre, .. } => {
                    reglas.insert(nombre.clone(), nodo);
                }
                _ => {}
            }
        }
    }

    match ejecutar_runtime(&ast, &mut environment, &funciones, &reglas, &mut simulador_robotico) {
        Ok(_) => {}
        Err(e) => { eprintln!("\n❌ ERROR DE EJECUCIÓN: {}", e); std::process::exit(1); }
    }
    let d_runtime = t_r0.elapsed().as_nanos();

    let tiempo_total_us = t_inicio.elapsed().as_micros();

    if config.modo_reporte == "detallado" {
        println!("\n=======================================================");
        println!(" ✅ PILOTO CORPORATIVO: RAMATI-Q (Fase 1.5) OPERATIVO");
        println!("=======================================================");
        println!(" -> Estatus:             [INTEGRIDAD_COMPLETA]");
        println!(" -> Kernel:              [{}]", config.version_kernel);
        println!(" -> Telemetría Capa 1:   {} nanos (Lexer)", d_lexer);
        println!(" -> Telemetría Capa 2:   {} nanos (Parser)", d_parser);
        println!(" -> Telemetría Capa 3:   {} nanos (Semántico)", d_semantico);
        println!(" -> Telemetría Capa 4:   {} nanos (Runtime)", d_runtime);
        println!(" -> Tiempo de CPU Total: {} microsegundos", tiempo_total_us);
        println!("=======================================================");
    }

    // --- PRUEBA DE MEMORIA CONTEXTUAL (v0.7) ---

// 1. Iniciamos la memoria para la sesión de Ramati-Q
let mut memoria = memoria_contextual::MemoriaContextual::origen("sesion_ramati_001");
println!("⏳ Iniciando sesión cognitiva: {}", memoria.id_sesion);

// 2. Simulamos que el sistema procesó datos y entró en Armonía
memoria.estado_actual = memoria_contextual::EstadoCognitivo::Armonia;
memoria.nivel_resonancia = 0.85;

// 3. Colapsamos la memoria en un archivo binario local
match memoria.colapsar_memoria("estado_ramati.qbin") {
    Ok(_) => println!(
        "✔️ Memoria persistida con éxito a la velocidad de la luz en estado_ramati.qbin"
    ),
    Err(e) => println!("❌ Error al guardar la memoria: {}", e),
}

// --- PRUEBA DE RECUPERACIÓN DE MEMORIA (v0.7) ---
println!("\n🔄 Intentando despertar memoria de sesiones anteriores...");

match memoria_contextual::MemoriaContextual::despertar_memoria("estado_ramati.qbin") {
    Ok(memoria_recuperada) => {
        println!("✔️ Memoria restaurada con éxito.");
        println!(" -> Sesión: {}", memoria_recuperada.id_sesion);
        println!(
            " -> Resonancia recuperada: {}",
            memoria_recuperada.nivel_resonancia
        );
    }
    Err(e) => println!(
        "⚠ No se encontró memoria previa o hubo un error: {}",
        e
    ),
}
}