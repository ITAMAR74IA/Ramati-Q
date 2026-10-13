use std::env;
use std::fs;
use std::process;
use serde::Deserialize; // <--- Importante para resolver el macro Deserialize

// Declaración de los módulos soberanos de Ramati-Q
mod ast;
mod entorno;
mod environment;
mod lexer;
mod parser;
mod semantic;
mod runtime;
mod hal; // <--- ¡Añadimos el módulo de hardware que faltaba declarar!
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
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        mostrar_ayuda();
        process::exit(0);
    }

    let comando = &args[1];

    match comando.as_str() {
        "run" => {
            if args.len() < 3 {
                eprintln!("[Ramati-Q Error] -> Faltan argumentos. Uso correcto: ramatiq run <archivo.rmq>");
                process::exit(1);
            }
            let ruta_archivo = &args[2];
            ejecutar_rmq(ruta_archivo);
        }
        "version" => {
            println!("Ramati-Q Compilador Soberano v0.1.0");
            println!("Núcleo: Cuántico-Difuso");
        }
        _ => {
            eprintln!("[Ramati-Q Error] -> Comando '{}' no reconocido.", comando);
            mostrar_ayuda();
            process::exit(1);
        }
    }
}

fn mostrar_ayuda() {
    println!("========================================");
    println!("        RAMATI-Q (Soberano v0.1.0)      ");
    println!("========================================");
    println!("Uso: ramatiq <comando> [archivo]");
    println!("Comandos disponibles:");
    println!("  run <archivo.rmq>   Ejecuta un programa escrito en Ramati-Q.");
    println!("  version             Muestra la versión actual del núcleo.");
    println!("========================================");
}

fn ejecutar_rmq(ruta: &String) {
    if !ruta.ends_with(".rmq") {
        eprintln!("[Ramati-Q Error] -> Archivo inválido. Ramati-Q solo ejecuta archivos '.rmq'.");
        process::exit(1);
    }

    match fs::read_to_string(ruta) {
        Ok(contenido) => {
            println!("[Ramati-Q Info] -> Iniciando ejecución de '{}'...", ruta);
            println!("----------------------------------------");
            
            // AQUÍ LLAMAMOS AL VERDADERO CEREBRO LÉXICO
            procesar_nucleo_oculto(&contenido);
            
            println!("----------------------------------------");
            println!("[Ramati-Q Info] -> Ejecución finalizada con éxito.");
        }
        Err(_) => {
            eprintln!("[Ramati-Q Error] -> No se pudo encontrar o leer el archivo: {}", ruta);
            process::exit(1);
        }
    }
}

// Función que procesa el código a través de todas las capas soberanas de Ramati-Q
fn procesar_nucleo_oculto(codigo: &str) {
    println!(">>> INVOCANDO EL NÚCLEO CUÁNTICO-DIFUSO DE RAMATI-Q <<<");
    
    // 1. Análisis Léxico
    let tokens = lexer::ejecutar_lexer(codigo);
    println!("[Ramati-Q Debug] -> Lexer: {} tokens extraídos.", tokens.len());
    
    // 2. Análisis Sintáctico (Parser)
    // Descomente o ajuste según la firma de su parser.rs
    // let ast = parser::ejecutar_parser(tokens);
    // println!("[Ramati-Q Debug] -> Parser: Árbol sintáctico generado con éxito.");

    // 3. Análisis Semántico
    // verificar_semantica(&ast);

    // 4. Ejecución en Runtime
    // ejecutar_runtime(ast);
    
    // Mostramos las fichas preliminares para verificar la lectura del main.rmq
    for (i, token) in tokens.iter().enumerate() {
        println!("  Ficha [{}]: {:?}", i + 1, token);
    }
}