// =====================================================================
// Módulo del Runtime (Motor de Ejecución) - Ramati-Q v1.5
// =====================================================================
// Este módulo es el corazón del Kernel. Toma el Árbol de Sintaxis
// Abstracto (AST) validado y lo ejecuta, interactuando con el entorno
// de ejecución y la capa de abstracción de hardware (HAL).

use crate::ast::*;
use crate::entorno::*;
use crate::hal::Hardware;

// Estructura para un Cubit Cognitivo (Fase 0.5)
pub struct CubitCognitivo {
    alpha: f64,
    beta: f64,
}

fn evaluar_expresion_difusa(expresion: &ExpresionLogica, env: &Environment, entorno: &std::collections::HashMap<String, CubitCognitivo>, base_de_hechos: &std::collections::HashMap<String, f64>) -> f64 {
    match expresion {
        ExpresionLogica::Comparacion { var, op, val } => {
            let p_actual = if let Some(valor_ramati) = env.get(var) {
                match valor_ramati {
                    ValorRamati::Espectro(valores) => valores.get(0).cloned().unwrap_or(0.0),
                    ValorRamati::Entero(v) => v as f64,
                    _ => 0.0,
                }
            } else if let Some(certeza_hecho) = base_de_hechos.get(var) {
                *certeza_hecho
            } else if let Some(cubit) = entorno.get(var) {
                cubit.beta * cubit.beta
            } else {
                0.0
            };

            let condicion_cumplida = match op.as_str() {
                ">"  => p_actual > *val,
                "<"  => p_actual < *val,
                ">=" => p_actual >= *val,
                "<=" => p_actual <= *val,
                "==" => (p_actual - *val).abs() < 1e-9,
                _ => false,
            };

            if condicion_cumplida {
                p_actual
            } else {
                0.0
            }
        },
        ExpresionLogica::Logica { expr1, op, expr2 } => {
            let certeza1 = evaluar_expresion_difusa(expr1, env, entorno, base_de_hechos);
            let certeza2 = evaluar_expresion_difusa(expr2, env, entorno, base_de_hechos);
            match op.as_str() {
                "y_difuso" => certeza1.min(certeza2),
                "o_difuso" => certeza1.max(certeza2),
                _ => 0.0,
            }
        }
    }
}

fn evaluar_expresion(
    exp: &Expresion,
    environment: &mut Environment,
    funciones: &std::collections::HashMap<String, &Nodo>, 
    reglas: &std::collections::HashMap<String, &Nodo>, 
    hardware: &mut dyn Hardware,
) -> Result<ValorRamati, String> {
    match exp {
        Expresion::Literal(val) => Ok(val.clone()),
        Expresion::Variable(nombre) => environment
            .get(nombre)
            .ok_or_else(|| format!("Error de ejecución: Variable no definida '{}'.", nombre)),
        Expresion::Agrupacion { expresion } => evaluar_expresion(expresion, environment, funciones, reglas, hardware),
        Expresion::Binaria { izquierda, operador, derecha } => {
            let val_izq = evaluar_expresion(izquierda, environment, funciones, reglas, hardware)?;
            let val_der = evaluar_expresion(derecha, environment, funciones, reglas, hardware)?;

            match (val_izq, val_der) {
                (ValorRamati::Entero(i1), ValorRamati::Entero(i2)) => {
                    match operador {
                        Token::Plus => Ok(ValorRamati::Entero(i1 + i2)),
                        Token::Minus => Ok(ValorRamati::Entero(i1 - i2)),
                        Token::Star => Ok(ValorRamati::Entero(i1 * i2)),
                        Token::Slash => {
                            if i2 == 0 { return Err("Error de ejecución: División por cero.".to_string()); }
                            Ok(ValorRamati::Entero(i1 / i2))
                        },
                        _ => Err("Operador binario no válido para enteros.".to_string()),
                    }
                }
                (ValorRamati::Espectro(v1), ValorRamati::Espectro(v2)) => {
                    let f1 = v1.get(0).cloned().unwrap_or(0.0);
                    let f2 = v2.get(0).cloned().unwrap_or(0.0);
                    let resultado = match operador {
                        Token::Plus => f1 + f2,
                        Token::Minus => f1 - f2,
                        Token::Star => f1 * f2,
                        Token::Slash => {
                            if f2.abs() < 1e-9 { return Err("Error de ejecución: División por cero.".to_string()); }
                            f1 / f2
                        },
                        _ => return Err("Operador binario no válido para espectros.".to_string()),
                    };
                    Ok(ValorRamati::Espectro(vec![resultado]))
                }
                _ => Err("Operación binaria no soportada para estos tipos de datos.".to_string()),
            }
        }
        Expresion::LlamadaFuncion { nombre, argumentos } => {
            if let Some(nodo_funcion) = funciones.get(nombre) {
                if let Nodo::DeclaracionFuncion { cuerpo, parametros, .. } = nodo_funcion {
                    if parametros.len() != argumentos.len() {
                        return Err(format!("Error de ejecución: La función '{}' esperaba {} argumentos, pero se proporcionaron {}.", nombre, parametros.len(), argumentos.len()));
                    }

                    let mut scope_funcion = Environment::new_enclosed(environment.clone());

                    for (param_nombre, arg_expr) in parametros.iter().zip(argumentos.iter()) {
                        let valor_argumento = evaluar_expresion(arg_expr, environment, funciones, reglas, hardware)?;
                        scope_funcion.define(param_nombre.clone(), valor_argumento);
                    }

                    match ejecutar_runtime(cuerpo, &mut scope_funcion, funciones, reglas, hardware)? {
                        Some(valor_retorno) => Ok(valor_retorno),
                        None => Ok(ValorRamati::Nulo),
                    }
                } else {
                    Err(format!("Error interno: El nodo para la función '{}' no es una DeclaracionFuncion.", nombre))
                }
            } else {
                Err(format!("Error de ejecución: Función '{}' no ha sido declarada.", nombre))
            }
        }
    }
}

pub fn ejecutar_runtime(
    ast: &Nodo, 
    environment: &mut Environment, 
    funciones: &std::collections::HashMap<String, &Nodo>, 
    reglas: &std::collections::HashMap<String, &Nodo>,
    hardware: &mut dyn Hardware,
) -> Result<Option<ValorRamati>, String> {
    if let Nodo::Programa(nodos) = ast {
        let mut entorno: std::collections::HashMap<String, CubitCognitivo> = std::collections::HashMap::new();
        let mut variables_observadas: Vec<String> = Vec::new();
        let _flag_seguridad: u8 = 0x00;
        
        for nodo in nodos {
            match nodo {
                Nodo::DeclaracionFuncion { .. } => (),
                Nodo::DeclaracionRegla { nombre, .. } => println!("[CONOCIMIENTO] 📖 Regla '{}' registrada en la base de conocimiento.", nombre),
                Nodo::ComandoRazonar => {
                    println!("[MOTOR COGNITIVO] Iniciando ciclo de inferencia (Forward Chaining)...");
                    let mut base_de_hechos: std::collections::HashMap<String, f64> = std::collections::HashMap::new();
                    let mut nuevos_hechos_generados = true;
                    let mut ciclo = 0;

                    while nuevos_hechos_generados {
                        ciclo += 1;
                        nuevos_hechos_generados = false;
                        println!("[MOTOR COGNITIVO] Ciclo de razonamiento #{}", ciclo);

                        for (nombre_regla, nodo_regla) in reglas {
                            if let Nodo::DeclaracionRegla { cuerpo, .. } = nodo_regla {
                                let grado_certeza = evaluar_expresion_difusa(cuerpo, environment, &entorno, &base_de_hechos);

                                if grado_certeza > 0.0 {
                                    let certeza_anterior = base_de_hechos.get(nombre_regla).cloned().unwrap_or(0.0);
                                    if grado_certeza > certeza_anterior {
                                        println!("[MOTOR COGNITIVO]   => Hecho derivado: '{}' con certeza {:.4} (supera {:.4})", nombre_regla, grado_certeza, certeza_anterior);
                                        base_de_hechos.insert(nombre_regla.clone(), grado_certeza);
                                        nuevos_hechos_generados = true;
                                    }
                                }
                            }
                        }
                    }

                    println!("[MOTOR COGNITIVO] Inferencia finalizada. Base de hechos consolidada:");
                    if base_de_hechos.is_empty() {
                        println!("  -> No se derivaron nuevos hechos.");
                    } else {
                        for (hecho, certeza) in &base_de_hechos {
                            println!("  -> Hecho: {:<20} | Certeza: {:.4}", hecho, certeza);
                        }
                    }
                }
                Nodo::ExpresionStatement { expresion } => {
                    evaluar_expresion(expresion, environment, funciones, reglas, hardware)?;
                }
                Nodo::Retornar { valor } => {
                    let valor_retorno = evaluar_expresion(valor, environment, funciones, reglas, hardware)?;
                    return Ok(Some(valor_retorno));
                }
                Nodo::DeclaracionEntero { nombre, valor } => {
                    let valor_evaluado = evaluar_expresion(valor, environment, funciones, reglas, hardware)?;
                    environment.define(nombre.clone(), valor_evaluado.clone());
                    println!("[KERNEL] 💾 Registro Nativo: Entero '{}' alojado en memoria con valor {}.", nombre, valor_evaluado);
                }
                Nodo::DeclaracionEspectro { nombre, valor } => {
                    let valor_evaluado = evaluar_expresion(valor, environment, funciones, reglas, hardware)?;
                    let num_valores = if let ValorRamati::Espectro(v) = &valor_evaluado { v.len() } else { 0 };
                    environment.define(nombre.clone(), valor_evaluado.clone());
                    println!("[KERNEL] 💾 Registro Nativo: Espectro '{}' alojado en memoria con {} valores.", nombre, num_valores);
                }
                Nodo::DeclaracionCadena { nombre, valor } => {
                    let valor_evaluado = evaluar_expresion(valor, environment, funciones, reglas, hardware)?;
                    if let ValorRamati::Cadena(s) = &valor_evaluado {
                        environment.define(nombre.clone(), valor_evaluado.clone());
                        println!("[KERNEL] 📝 Registro Nativo: Cadena '{}' alojada en memoria con valor '{}'.", nombre, s);
                    } else { return Err(format!("Error de ejecución: Se esperaba una cadena para la variable '{}'.", nombre)); }
                }
                Nodo::ComandoMostrar(expresion) => {
                    let valor_a_mostrar = evaluar_expresion(expresion, environment, funciones, reglas, hardware)?;
                    println!("{}", valor_a_mostrar);
                }
                Nodo::UsarModulo(nombre) => {
                    let nombre_archivo = format!("{}.rmq", nombre);
                    println!("[KERNEL] 📦 Buscando archivo de módulo físico: '{}'...", nombre_archivo);

                    match std::fs::read_to_string(&nombre_archivo) {
                        Ok(contenido) => {
                            println!("  ✔️ Módulo '{}' localizado ({} bytes).", nombre_archivo, contenido.len());
                            println!("[KERNEL] ⚙️ Compilando e inyectando módulo en tiempo real...");

                            let tokens_modulo = crate::lexer::ejecutar_lexer(&contenido);
                            match crate::parser::ejecutar_parser(&tokens_modulo) {
                                Ok(ast_modulo) => {
                                    if let Err(e) = ejecutar_runtime(&ast_modulo, environment, funciones, reglas, hardware).map(|_| ()) {
                                        println!("❌ ERROR AL EJECUTAR MÓDULO '{}': {}", nombre_archivo, e);
                                    } else {
                                        println!("  ✔ Módulo '{}' ejecutado correctamente.", nombre_archivo);
                                    }
                                }
                                Err(e) => println!("❌ ERROR DE SINTAXIS EN MÓDULO '{}': {}", nombre_archivo, e),
                            }
                        }
                        Err(_) => {
                            println!("❌ ERROR DE RUNTIME: El módulo '{}' no existe en el directorio actual. Verifique la ruta.", nombre_archivo);
                        }
                    }
                }
                Nodo::DeclaracionCubit { nombre, probabilidad } => {
                    let beta = probabilidad.sqrt();
                    let alpha = (1.0 - probabilidad).sqrt();
                    entorno.insert(nombre.clone(), CubitCognitivo { alpha, beta });
                }
                Nodo::ObservarCubit(nombre) => {
                    if !variables_observadas.contains(nombre) {
                        variables_observadas.push(nombre.clone());
                    }
                }
                Nodo::CapturarDato { variable, canal } => {
                    let valor_sensor = hardware.capturar(canal);
                    environment.define(variable.clone(), valor_sensor);
                }
                Nodo::ArmonizarSistema => {}
                Nodo::EntrelazarCanalesFisicos(_canal) => {}
                Nodo::CondicionalDifuso { expresion, cuerpo_si, cuerpo_sino } => {
                    let grado_certeza = evaluar_expresion_difusa(expresion, &environment, &entorno, &std::collections::HashMap::new());
                    if grado_certeza > 0.0 {
                        println!("[INFERENCIA] Condición cumplida con certeza {:.4}. Ejecutando bloque de acción...", grado_certeza);
                        ejecutar_runtime(cuerpo_si, environment, funciones, reglas, hardware)?;
                    } else {
                        if let Some(cuerpo) = cuerpo_sino {
                            ejecutar_runtime(cuerpo, environment, funciones, reglas, hardware)?;
                        }
                    }
                }
                Nodo::ComandoAccionar { actuador, valor } => {
                    let valor_evaluado = evaluar_expresion(valor, environment, funciones, reglas, hardware)?;
                    if let ValorRamati::Espectro(v) = valor_evaluado {
                        let val_f64 = v.get(0).cloned().unwrap_or(0.0);
                        hardware.accionar(actuador, val_f64);
                    }
                }
                Nodo::IntersecarVariables { destino, var1, var2 } => {
                    let p1 = entorno.get(var1).map(|c| c.beta * c.beta).unwrap_or(0.0);
                    let p2 = entorno.get(var2).map(|c| c.beta * c.beta).unwrap_or(0.0);
                    let p_intersección = p1.min(p2);
                    if let Some(dest) = entorno.get_mut(destino) {
                        dest.beta = p_intersección.sqrt();
                        dest.alpha = (1.0_f64 - p_intersección).sqrt();
                    }
                }
                Nodo::EntrelazarVariables { var1, var2 } => {
                    let p_control = entorno.get(var1).map(|c| c.beta * c.beta).unwrap_or(0.0);
                    if p_control > 0.50 {
                        if let Some(obj) = entorno.get_mut(var2) {
                            std::mem::swap(&mut obj.alpha, &mut obj.beta);
                        }
                        println!("[CUÁNTICO] Compuerta CNOT ejecutada con éxito. '{}' enlazado con '{}'.", var1, var2);
                    }
                }
                Nodo::ResonarBucleSistema { var1, var2, delta } => {
                    println!("[HOMEÓSTASIS] Ejecutando bucle de resonancia industrial. Delta objetivo <= {}", delta);
                    if let Some(c1) = entorno.get_mut(var1) {
                        c1.beta = (*delta).sqrt();
                        c1.alpha = (1.0 - *delta).sqrt();
                    }
                    if let Some(c2) = entorno.get_mut(var2) {
                        c2.beta = (1.0 - *delta).sqrt();
                        c2.alpha = (*delta).sqrt();
                    }
                }
                Nodo::SuperponerCubit(nombre) => {
                    if let Some(c) = entorno.get_mut(nombre) {
                        c.alpha = 1.0 / 2.0_f64.sqrt();
                        c.beta = 1.0 / 2.0_f64.sqrt();
                    }
                }
                Nodo::SintonizacionCubit { nombre, probabilidad } => {
                    if *probabilidad < 0.0 || *probabilidad > 1.0 {
                        return Err(format!("❌ ERROR DE EJECUCIÓN: Probabilidad fuera de rango para '{}' ({}).", nombre, probabilidad));
                    }

                    if let Some(c) = entorno.get_mut(nombre) {
                        c.beta = probabilidad.sqrt();
                        c.alpha = (1.0 - *probabilidad).sqrt();
                    }
                }
                Nodo::FluctuarCubit(nombre) => {
                    if let Some(c) = entorno.get_mut(nombre) {
                        std::mem::swap(&mut c.alpha, &mut c.beta);
                    }
                }
                Nodo::MutarSistema => {
                    println!("[KERNEL] 🧬 Mutación del sistema iniciada (placeholder).");
                }
                Nodo::DiagnosticoSistema => {
                    println!("\n=== DIAGNÓSTICO COGNITIVO DE VARIABLES EN CALIENTE ===");
                    for (nombre, cubit) in &entorno {
                        println!(" -> Variable: {:<20} | Probabilidad Activa (|ß|²): {:.4}", nombre, cubit.beta * cubit.beta);
                    }
                    println!("=====================================================\n");
                }
                Nodo::DiagnosticoHardware => {
                    println!("{}", hardware.obtener_diagnostico());
                }
                Nodo::ComandoObjetivo { x, y } => {
                    let val_x = evaluar_expresion(x, environment, funciones, reglas, hardware)?;
                    let val_y = evaluar_expresion(y, environment, funciones, reglas, hardware)?;
                    if let (ValorRamati::Espectro(vx), ValorRamati::Espectro(vy)) = (&val_x, &val_y) {
                        hardware.establecer_objetivo(vx[0], vy[0]);
                    } else {
                         return Err("Error de ejecución: Las coordenadas del objetivo deben ser numéricas.".to_string());
                    }
                }
                Nodo::BucleCognitivo { ciclos, cuerpo } => {
                    if let ValorRamati::Entero(num_ciclos) = evaluar_expresion(ciclos, environment, funciones, reglas, hardware)? {
                        println!("[BUCLE COGNITIVO] Iniciando bucle de {} ciclos de percepción-acción.", num_ciclos);
                        for ciclo_actual in 1..=num_ciclos {
                            println!("\n--- Ciclo de Homeostasis #{} ---", ciclo_actual);
                            ejecutar_runtime(cuerpo, environment, funciones, reglas, hardware)?;
                            hardware.paso_fisico(0.1);
                            if hardware.objetivo_alcanzado() {
                                println!("[BUCLE COGNITIVO] Misión completada. Terminando bucle.");
                                break;
                            }
                        }
                        println!("[BUCLE COGNITIVO] Bucle finalizado.");
                    } else {
                        return Err("Error de ejecución: El número de ciclos para 'bucle_cognitivo' debe ser un entero.".to_string());
                    }
                }
                _ => {}
            }
        }

        let mut suma_probabilidades = 0.0;
        let total_observadas = variables_observadas.len();

        for var in &variables_observadas {
            if let Some(c) = entorno.get(var) {
                suma_probabilidades += c.beta * c.beta;
            }
        }

        let certeza_consolidada = if total_observadas > 0 {
            suma_probabilidades / total_observadas as f64
        } else {
            0.0
        };

        use std::io::Write;
        let mut archivo_binario = std::fs::File::create("resultado.qbin").map_err(|e| e.to_string())?;
        archivo_binario.write_all(&[0x52, 0x4D, 0x51, 0x00]).map_err(|e| e.to_string())?;
        archivo_binario.write_all(&[_flag_seguridad]).map_err(|e| e.to_string())?;
        let certeza_f32 = certeza_consolidada as f32;
        archivo_binario.write_all(&certeza_f32.to_be_bytes()).map_err(|e| e.to_string())?;

        println!("[CAJA NEGRA] Archivo 'resultado.qbin' guardado con éxito (9 Bytes). Flag de Emergencia: 0x{:02X}, Certeza Consolidada: {:.4}", _flag_seguridad, certeza_consolidada);
    }    Ok(None)
}