// =====================================================================
// Módulo del Analizador Semántico - Ramati-Q v1.5
// =====================================================================
// Este módulo recorre el Árbol de Sintaxis Abstracto (AST) para
// verificar la coherencia lógica del código antes de la ejecución.
// Por ejemplo, comprueba que las variables se usen después de ser
// declaradas.

use crate::ast::*;
use std::collections::HashSet;

pub fn verificar_semantica(ast: &Nodo) -> Result<(), String> {
    if let Nodo::Programa(nodos) = ast {
        let mut variables_inicializadas: HashSet<String> = HashSet::new();

        for nodo in nodos {
            match nodo {
                Nodo::DeclaracionEntero { nombre, .. } | 
                Nodo::DeclaracionCadena { nombre, .. } | 
                Nodo::DeclaracionRegla { nombre, .. } |
                Nodo::DeclaracionEspectro { nombre, .. } => {
                    variables_inicializadas.insert(nombre.clone());
                }

                Nodo::DeclaracionCubit { nombre, probabilidad } => {
                    if *probabilidad < 0.0 || *probabilidad > 1.0 {
                        return Err(format!("❌ ERROR SEMÁNTICO: Espectro inválido para '{}' ({}).", nombre, probabilidad));
                    }
                    variables_inicializadas.insert(nombre.clone());
                }

                Nodo::ComandoMostrar(_expresion) => {
                    // La validación se hará en el runtime.
                }

                Nodo::SintonizacionCubit { nombre, probabilidad } => {
                    if !variables_inicializadas.contains(nombre) {
                        return Err(format!("❌ ERROR SEMÁNTICO CRÍTICO: Variable fantasma '{}'.", nombre));
                    }
                    if *probabilidad < 0.0 || *probabilidad > 1.0 {
                        return Err(format!("❌ ERROR SEMÁNTICO: Espectro inválido para '{}'.", nombre));
                    }
                }

                Nodo::ObservarCubit(nombre) | Nodo::FluctuarCubit(nombre) | Nodo::SuperponerCubit(nombre) => {
                    if !variables_inicializadas.contains(nombre) {
                        return Err(format!("❌ ERROR SEMÁNTICO: Variable fantasma '{}' detectada.", nombre));
                    }
                }

                Nodo::CapturarDato { variable, .. } => {
                    if !variables_inicializadas.contains(variable) {
                        return Err(format!("❌ ERROR SEMÁNTICO: Variable no declarada '{}'.", variable));
                    }
                }

                Nodo::IntersecarVariables { destino, var1, var2 } => {
                    if !variables_inicializadas.contains(destino) {
                        return Err(format!("❌ ERROR SEMÁNTICO: Variable de destino fantasma '{}'.", destino));
                    }
                    if !variables_inicializadas.contains(var1) || !variables_inicializadas.contains(var2) {
                        return Err(format!("❌ ERROR SEMÁNTICO: Intersección en variables fantasmas."));
                    }
                }

                Nodo::EntrelazarVariables { var1, var2 } | Nodo::ResonarBucleSistema { var1, var2, .. } => {
                    if !variables_inicializadas.contains(var1) || !variables_inicializadas.contains(var2) {
                        return Err(format!("❌ ERROR SEMÁNTICO: Operación cuántica en variables fantasmas."));
                    }
                }

                Nodo::CondicionalDifuso { expresion, .. } => {
                    fn verificar_expresion(expresion: &ExpresionLogica, variables_inicializadas: &HashSet<String>) -> Result<(), String> {
                        match expresion {
                            ExpresionLogica::Comparacion { var, .. } => {
                                if !variables_inicializadas.contains(var) {
                                    return Err(format!("❌ ERROR SEMÁNTICO: Evaluación sobre variable fantasma '{}'.", var));
                                }
                            },
                            ExpresionLogica::Logica { expr1, expr2, .. } => {
                                verificar_expresion(expr1, variables_inicializadas)?;
                                verificar_expresion(expr2, variables_inicializadas)?;
                            }
                        }
                        Ok(())
                    }
                    if let Err(e) = verificar_expresion(expresion, &variables_inicializadas) {
                        return Err(e);
                    }
                }
                
                _ => {} 
            }
        }
    }
    Ok(())
}