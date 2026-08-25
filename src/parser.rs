// =====================================================================
// Módulo del Parser (Analizador Sintáctico) - Ramati-Q v1.5
// =====================================================================
// Este módulo toma la secuencia de Tokens del Lexer y la convierte en
// un Árbol de Sintaxis Abstracto (AST), que es una representación
// estructurada y jerárquica del código.

use crate::ast::*;
use crate::entorno::ValorRamati;

fn parse_primary(tokens: &[Token], i: &mut usize) -> Result<Expresion, String> {
    let token = tokens.get(*i).ok_or("Error de sintaxis: Expresión inesperada al final del archivo.")?;
    match token {
        Token::CadenaTexto(s) => {
            *i += 1;
            Ok(Expresion::Literal(ValorRamati::Cadena(s.clone())))
        }
        Token::NumeroEntero(n) => {
            *i += 1;
            Ok(Expresion::Literal(ValorRamati::Entero(*n)))
        }
        Token::Probabilidad(f) => {
            *i += 1;
            Ok(Expresion::Literal(ValorRamati::Espectro(vec![*f])))
        }
        Token::Minus => {
            *i += 1; // Consumir '-'
            let next_token = tokens.get(*i).ok_or("Error de sintaxis: Expresión incompleta después de signo '-'.".to_string())?;
            match next_token {
                Token::NumeroEntero(n) => {
                    *i += 1;
                    Ok(Expresion::Literal(ValorRamati::Entero(-*n)))
                }
                Token::Probabilidad(f) => {
                    *i += 1;
                    Ok(Expresion::Literal(ValorRamati::Espectro(vec![-*f])))
                }
                _ => Err("Error de sintaxis: Se esperaba un valor numérico después del signo '-'.".to_string())
            }
        }
        Token::ParentesisAbre => {
            *i += 1; // Consumir '('
            let expr = parse_expression(tokens, i)?;
            if tokens.get(*i) != Some(&Token::ParentesisCierra) {
                return Err("Error de sintaxis: Se esperaba ')' para cerrar la expresión.".to_string());
            }
            *i += 1; // Consumir ')'
            Ok(Expresion::Agrupacion { expresion: Box::new(expr) })
        }
        Token::Identificador(id) => {
            // Podría ser una variable o una llamada a función
            if tokens.get(*i + 1) == Some(&Token::ParentesisAbre) {
                let nombre = id.clone();
                let mut current_pos = *i + 2; // After '('
                let mut argumentos = Vec::new();

                if tokens.get(current_pos) != Some(&Token::ParentesisCierra) {
                    loop {
                        let arg_expr = parse_expression(tokens, &mut current_pos)?;
                        argumentos.push(arg_expr);
                        match tokens.get(current_pos) {
                            Some(Token::Coma) => { current_pos += 1; }
                            Some(Token::ParentesisCierra) => { break; }
                            _ => return Err("Error de sintaxis: Se esperaba ',' o ')' en la lista de argumentos.".to_string()),
                        }
                    }
                }
                if tokens.get(current_pos) != Some(&Token::ParentesisCierra) { return Err("Error de sintaxis: La lista de argumentos no se cierra con ')'.".to_string()); }
                current_pos += 1;
                *i = current_pos;
                Ok(Expresion::LlamadaFuncion { nombre, argumentos })
            } else {
                // Es una variable
                *i += 1;
                Ok(Expresion::Variable(id.clone()))
            }
        }
        _ => Err("Error de sintaxis: Se esperaba una expresión (literal, variable o llamada a función).".to_string()),
    }
}

fn parse_factor(tokens: &[Token], i: &mut usize) -> Result<Expresion, String> {
    let mut expr = parse_primary(tokens, i)?;
    while let Some(token) = tokens.get(*i) {
        match token {
            Token::Star | Token::Slash => {
                let operador = token.clone();
                *i += 1;
                let derecha = parse_primary(tokens, i)?;
                expr = Expresion::Binaria {
                    izquierda: Box::new(expr),
                    operador,
                    derecha: Box::new(derecha),
                };
            }
            _ => break,
        }
    }
    Ok(expr)
}

fn parse_term(tokens: &[Token], i: &mut usize) -> Result<Expresion, String> {
    let mut expr = parse_factor(tokens, i)?;
    while let Some(token) = tokens.get(*i) {
        match token {
            Token::Plus | Token::Minus => {
                let operador = token.clone();
                *i += 1;
                let derecha = parse_factor(tokens, i)?;
                expr = Expresion::Binaria {
                    izquierda: Box::new(expr),
                    operador,
                    derecha: Box::new(derecha),
                };
            }
            _ => break,
        }
    }
    Ok(expr)
}

fn parse_expression(tokens: &[Token], i: &mut usize) -> Result<Expresion, String> {
    parse_term(tokens, i)
}

pub fn ejecutar_parser(tokens: &[Token]) -> Result<Nodo, String> {
    let mut nodos_internos = Vec::new();
    let mut i = 0;

    if !tokens.is_empty() && tokens[0] == Token::Origen {
        i += 1; 
    } else if !tokens.is_empty() && tokens[0] != Token::EndFunc && tokens.iter().any(|t| t == &Token::Colapsar) {
        return Err("❌ ERROR CRÍTICO DE GRAMÁTICA: El punto de entrada 'origen' es obligatorio.".to_string());
    }

    while i < tokens.len() {
        match &tokens[i] {
            Token::Usar => {
                if i + 1 < tokens.len() {
                    if let Token::Identificador(nombre_mod) = &tokens[i+1] {
                        nodos_internos.push(Nodo::UsarModulo(nombre_mod.clone()));
                        i += 1;
                    } else {
                        return Err(format!("❌ ERROR DE SINTAXIS: Se esperaba un identificador después de 'usar', se encontró '{:?}'", tokens[i+1]));
                    }
                } else {
                    return Err("❌ ERROR DE SINTAXIS: Se esperaba un identificador después de 'usar'.".to_string());
                }
            }
            Token::VariableDifusa => {
                if i + 2 < tokens.len() {
                    if let (Token::Identificador(id), Token::Probabilidad(prob)) = (&tokens[i+1], &tokens[i+2]) {
                        nodos_internos.push(Nodo::DeclaracionCubit { nombre: id.clone(), probabilidad: *prob });
                        i += 2;
                    }
                }
            }
            Token::TipoEntero => {
                if i + 2 < tokens.len() {
                    if let (Token::Identificador(id), Token::OperadorRelacional(op)) = (&tokens[i+1], &tokens[i+2]) {
                        if op == "=" {
                            let mut expr_idx = i + 3;
                            let valor_expr = parse_expression(tokens, &mut expr_idx)?;
                            nodos_internos.push(Nodo::DeclaracionEntero { nombre: id.clone(), valor: valor_expr });
                            i = expr_idx - 1;
                        } else {
                             return Err(format!("❌ ERROR DE SINTAXIS: Se esperaba '=' en la declaración de entero '{}', se encontró '{}'", id, op));
                        }
                    } else {
                        return Err("❌ ERROR DE SINTAXIS: Formato incorrecto para declaración de entero. Se esperaba 'entero <nombre> = <valor>'".to_string());
                    }
                } else {
                    return Err("❌ ERROR DE SINTAXIS: Declaración de 'entero' incompleta.".to_string());
                }
            }
            Token::TipoCadena => {
                if i + 2 < tokens.len() {
                    if let (Token::Identificador(id), Token::OperadorRelacional(op)) = (&tokens[i+1], &tokens[i+2]) {
                        if op == "=" {
                            let mut expr_idx = i + 3;
                            let valor_expr = parse_expression(tokens, &mut expr_idx)?;
                            nodos_internos.push(Nodo::DeclaracionCadena { nombre: id.clone(), valor: valor_expr });
                            i = expr_idx - 1;
                        } else {
                            return Err(format!("❌ ERROR DE SINTAXIS: Se esperaba '=' en la declaración de cadena '{}', se encontró '{}'", id, op));
                        }
                    } else {
                        return Err("❌ ERROR DE SINTAXIS: Formato incorrecto para declaración de cadena. Se esperaba 'cadena <nombre> = \"valor\"'".to_string());
                    }
                } else {
                    return Err("❌ ERROR DE SINTAXIS: Declaración de 'cadena' incompleta.".to_string());
                }
            }
            Token::TipoEspectro => {
                if i + 2 < tokens.len() {
                    if let (Token::Identificador(id), Token::OperadorRelacional(op)) = (&tokens[i+1], &tokens[i+2]) {
                        if op == "=" {
                            let mut expr_idx = i + 3;
                            let valor_expr = parse_expression(tokens, &mut expr_idx)?;
                            nodos_internos.push(Nodo::DeclaracionEspectro { nombre: id.clone(), valor: valor_expr });
                            i = expr_idx - 1;
                        }
                    }
                } else {
                    return Err("❌ ERROR DE SINTAXIS: Declaración de 'espectro' incompleta.".to_string());
                }
            }
            Token::Sintonizar => {
                if i + 2 < tokens.len() {
                    if let (Token::Identificador(id), Token::Probabilidad(prob)) = (&tokens[i+1], &tokens[i+2]) {
                        nodos_internos.push(Nodo::SintonizacionCubit { nombre: id.clone(), probabilidad: *prob });
                        i += 2;
                    }
                }
            }
            Token::Mostrar => {
                if i + 1 < tokens.len() {
                    let mut expr_idx = i + 1;
                    match parse_expression(tokens, &mut expr_idx) {
                        Ok(expr) => {
                            nodos_internos.push(Nodo::ComandoMostrar(expr));
                            i = expr_idx - 1;
                        }
                        Err(e) => return Err(e),
                    }
                }
            }
            Token::Observar => {
                if i + 1 < tokens.len() {
                    if let Token::Identificador(id) = &tokens[i+1] {
                        nodos_internos.push(Nodo::ObservarCubit(id.clone()));
                        i += 1;
                    }
                }
            }
            Token::Capturar => {
                if i + 3 < tokens.len() {
                    if let (Token::Identificador(var), Token::De, Token::Identificador(sensor)) = (&tokens[i+1], &tokens[i+2], &tokens[i+3]) {
                        nodos_internos.push(Nodo::CapturarDato { variable: var.clone(), canal: sensor.clone() });
                        i += 3;
                    } else {
                        return Err("Error de sintaxis: Se esperaba 'capturar <variable> de <sensor>'".to_string());
                    }
                } else {
                    return Err("Error de sintaxis: Comando 'capturar' incompleto.".to_string());
                }
            }
            Token::Intersecar => {
                if i + 3 < tokens.len() {
                    if let (Token::Identificador(dest), Token::Identificador(v1), Token::Identificador(v2)) = (&tokens[i+1], &tokens[i+2], &tokens[i+3]) {
                        nodos_internos.push(Nodo::IntersecarVariables { destino: dest.clone(), var1: v1.clone(), var2: v2.clone() });
                        i += 3;
                    }
                }
            }
            Token::Entrelazar => {
                if i + 2 < tokens.len() {
                    if let (Token::Identificador(v1), Token::Identificador(v2)) = (&tokens[i+1], &tokens[i+2]) {
                        nodos_internos.push(Nodo::EntrelazarVariables { var1: v1.clone(), var2: v2.clone() });
                        i += 2;
                    }
                }
            }
            Token::ResonarBucle => {
                if i + 4 < tokens.len() {
                    if let (Token::Identificador(v1), Token::Identificador(v2), Token::Hasta, Token::Probabilidad(d)) = (&tokens[i+1], &tokens[i+2], &tokens[i+3], &tokens[i+4]) {
                        nodos_internos.push(Nodo::ResonarBucleSistema { var1: v1.clone(), var2: v2.clone(), delta: *d });
                        i += 4;
                    }
                }
            }
            Token::SiDifuso => {
                let start_of_expr = i + 1;
                let mut end_of_expr = start_of_expr;

                while end_of_expr < tokens.len() && tokens[end_of_expr] != Token::Flecha {
                    end_of_expr += 1;
                }

                if end_of_expr >= tokens.len() {
                    return Err("Error de sintaxis: 'si_difuso' sin bloque de acción '->'".to_string());
                }

                match parse_fuzzy_expression(&tokens[start_of_expr..end_of_expr]) {
                    Ok((expr_tree, _)) => {
                        let mut current_pos = end_of_expr;

                        if current_pos < tokens.len() && tokens[current_pos] == Token::Flecha {
                            current_pos += 1;
                            
                            let body_start_idx = current_pos;
                            let mut body_end_idx = body_start_idx;
                            
                            while body_end_idx < tokens.len() && tokens[body_end_idx] != Token::FinSi && tokens[body_end_idx] != Token::Sino {
                                body_end_idx += 1;
                            }

                            if body_end_idx >= tokens.len() {
                                return Err("Error de sintaxis: Bloque 'si_difuso' no cerrado con 'fin_si'.".to_string());
                            }

                            let cuerpo_si = ejecutar_parser(&tokens[body_start_idx..body_end_idx])?;
                            
                            let cuerpo_sino = None;

                            current_pos = body_end_idx;

                            nodos_internos.push(Nodo::CondicionalDifuso {
                                expresion: expr_tree,
                                cuerpo_si: Box::new(cuerpo_si),
                                cuerpo_sino,
                            });
                            i = current_pos;
                        }
                    }
                    Err(e) => return Err(e),
                }
            }
            Token::Armonizar => nodos_internos.push(Nodo::ArmonizarSistema),
            Token::Mutar => nodos_internos.push(Nodo::MutarSistema),
            Token::Diagnostico => nodos_internos.push(Nodo::DiagnosticoSistema),
            Token::DiagnosticoHardware => nodos_internos.push(Nodo::DiagnosticoHardware),
            Token::Accionar => {
                if i + 3 < tokens.len() {
                    if let (Token::Identificador(actuador), Token::Con) = (&tokens[i+1], &tokens[i+2]) {
                        let mut expr_idx = i + 3;
                        let valor_expr = parse_expression(tokens, &mut expr_idx)?;
                        nodos_internos.push(Nodo::ComandoAccionar { actuador: actuador.clone(), valor: valor_expr });
                        i = expr_idx - 1;
                    } else {
                        return Err("Error de sintaxis: Se esperaba 'accionar <actuador> con <valor>'".to_string());
                    }
                } else {
                    return Err("Error de sintaxis: Comando 'accionar' incompleto.".to_string());
                }
            }
            Token::EntrelazarCanales => {
                if i + 1 < tokens.len() {
                    if let Token::Identificador(id) = &tokens[i+1] {
                        nodos_internos.push(Nodo::EntrelazarCanalesFisicos(id.clone()));
                        i += 1;
                    }
                }
            }
            Token::Fluctuar => {
                if i + 1 < tokens.len() {
                    if let Token::Identificador(id) = &tokens[i+1] {
                        nodos_internos.push(Nodo::FluctuarCubit(id.clone()));
                        i += 1;
                    }
                }
            }
            Token::Superponer => {
                if i + 1 < tokens.len() {
                    if let Token::Identificador(id) = &tokens[i+1] {
                        nodos_internos.push(Nodo::SuperponerCubit(id.clone()));
                        i += 1;
                    }
                }
            }
            Token::Func => {
                if i + 3 >= tokens.len() || !matches!(&tokens[i+1], Token::Identificador(_)) || tokens[i+2] != Token::ParentesisAbre {
                    return Err("Error de sintaxis: Se esperaba 'funcion <nombre>()'".to_string());
                }
                let nombre_func = if let Token::Identificador(n) = &tokens[i+1] { n.clone() } else { unreachable!() };
                
                let mut current_pos = i + 3;
                let mut parametros = Vec::new();

                if tokens.get(current_pos) != Some(&Token::ParentesisCierra) {
                    loop {
                        if let Some(Token::Identificador(param_name)) = tokens.get(current_pos) {
                            parametros.push(param_name.clone());
                            current_pos += 1;
                        } else {
                            return Err("Error de sintaxis: Se esperaba un identificador de parámetro.".to_string());
                        }

                        match tokens.get(current_pos) {
                            Some(Token::Coma) => {
                                current_pos += 1;
                            }
                            Some(Token::ParentesisCierra) => {
                                break;
                            }
                            _ => {
                                return Err("Error de sintaxis: Se esperaba ',' o ')' después de un parámetro.".to_string());
                            }
                        }
                    }
                }
                
                if tokens.get(current_pos) != Some(&Token::ParentesisCierra) {
                     return Err("Error de sintaxis: La lista de parámetros no se cierra con ')'.".to_string());
                }
                current_pos += 1;

                let body_start_idx = current_pos;
                let mut body_end_idx = body_start_idx;
                let mut nesting_level = 1;

                while body_end_idx < tokens.len() {
                    if tokens[body_end_idx] == Token::Func {
                        nesting_level += 1;
                    } else if tokens[body_end_idx] == Token::EndFunc {
                        nesting_level -= 1;
                        if nesting_level == 0 {
                            break;
                        }
                    }
                    body_end_idx += 1;
                }

                if nesting_level != 0 {
                    return Err(format!("Error de sintaxis: La función '{}' no tiene un 'fin_funcion' correspondiente.", nombre_func));
                }

                let body_tokens = &tokens[body_start_idx..body_end_idx];
                let ast_cuerpo = ejecutar_parser(body_tokens)?;

                nodos_internos.push(Nodo::DeclaracionFuncion {
                    nombre: nombre_func,
                    parametros,
                    cuerpo: Box::new(ast_cuerpo),
                });

                i = body_end_idx + 1;
                continue;
            }
            Token::Identificador(nombre) => {
                let mut current_pos = i;
                match parse_expression(tokens, &mut current_pos) {
                    Ok(expr) => {
                        if matches!(expr, Expresion::LlamadaFuncion {..}) {
                            nodos_internos.push(Nodo::ExpresionStatement { expresion: expr });
                            i = current_pos - 1;
                        } else {
                            return Err(format!("Error de sintaxis: Identificador suelto '{}' no es una instrucción válida.", nombre));
                        }
                    }
                    Err(e) => return Err(e),
                }
            }
            Token::Return => {
                let mut expr_idx = i + 1;
                match parse_expression(tokens, &mut expr_idx) {
                    Ok(expr) => {
                        nodos_internos.push(Nodo::Retornar { valor: expr });
                        i = expr_idx - 1;
                    }
                    Err(e) => return Err(e),
                }
            }
            Token::Regla => {
                if i + 1 >= tokens.len() || !matches!(&tokens[i+1], Token::Identificador(_)) {
                    return Err("Error de sintaxis: Se esperaba 'regla <nombre>'".to_string());
                }
                let nombre_regla = if let Token::Identificador(n) = &tokens[i+1] { n.clone() } else { unreachable!() };

                let body_start_idx = i + 2;
                let mut body_end_idx = body_start_idx;

                while body_end_idx < tokens.len() {
                    if tokens[body_end_idx] == Token::FinRegla {
                        break;
                    }
                    body_end_idx += 1;
                }

                if body_end_idx >= tokens.len() || tokens[body_end_idx] != Token::FinRegla {
                    return Err(format!("Error de sintaxis: La regla '{}' no tiene un 'fin_regla' correspondiente.", nombre_regla));
                }

                let (expr_tree, _) = parse_fuzzy_expression(&tokens[body_start_idx..body_end_idx])?;

                nodos_internos.push(Nodo::DeclaracionRegla {
                    nombre: nombre_regla,
                    cuerpo: expr_tree,
                });

                i = body_end_idx + 1;
                continue;
            }
            Token::Razonar => {
                nodos_internos.push(Nodo::ComandoRazonar);
            }
            Token::Colapsar => break,
            Token::Objetivo => {
                if i + 2 < tokens.len() {
                    let mut expr_idx_x = i + 1;
                    let x_expr = parse_expression(tokens, &mut expr_idx_x)?;
                    let mut expr_idx_y = expr_idx_x;
                    let y_expr = parse_expression(tokens, &mut expr_idx_y)?;
                    nodos_internos.push(Nodo::ComandoObjetivo { x: x_expr, y: y_expr });
                    i = expr_idx_y - 1;
                } else {
                    return Err("Error de sintaxis: Se esperaba 'objetivo <x> <y>'".to_string());
                }
            }
            Token::BucleCognitivo => {
                let mut expr_idx = i + 1;
                let ciclos_expr = parse_expression(tokens, &mut expr_idx)?;

                if tokens.get(expr_idx) != Some(&Token::Flecha) {
                    return Err("Error de sintaxis: Se esperaba '->' después del número de ciclos en 'bucle_cognitivo'.".to_string());
                }
                expr_idx += 1;

                let body_start_idx = expr_idx;
                let mut body_end_idx = body_start_idx;
                let mut nesting_level = 1;

                while body_end_idx < tokens.len() {
                    if tokens[body_end_idx] == Token::BucleCognitivo { nesting_level += 1; }
                    else if tokens[body_end_idx] == Token::FinBucle {
                        nesting_level -= 1;
                        if nesting_level == 0 { break; }
                    }
                    body_end_idx += 1;
                }

                if nesting_level != 0 {
                    return Err("Error de sintaxis: 'bucle_cognitivo' no tiene un 'fin_bucle' correspondiente.".to_string());
                }

                let cuerpo_bucle = ejecutar_parser(&tokens[body_start_idx..body_end_idx])?;
                nodos_internos.push(Nodo::BucleCognitivo { ciclos: ciclos_expr, cuerpo: Box::new(cuerpo_bucle) });

                i = body_end_idx;
            }
            _ => {}
        }
        i += 1;
    }

    Ok(Nodo::Programa(nodos_internos))
}

fn parse_fuzzy_expression(tokens: &[Token]) -> Result<(ExpresionLogica, usize), String> {
    let mut current_pos = 0;
    let mut expresiones = Vec::new();
    let mut operadores = Vec::new();

    loop {
        if current_pos + 2 < tokens.len() {
            if let (Token::Identificador(v), Token::OperadorRelacional(op)) = (&tokens[current_pos], &tokens[current_pos + 1]) {
                let mut val: Option<f64> = None;
                let mut consumed = 0;

                if tokens[current_pos + 2] == Token::Minus {
                    if current_pos + 3 < tokens.len() {
                        val = match &tokens[current_pos + 3] {
                            Token::Probabilidad(p) => Some(-*p),
                            Token::NumeroEntero(n) => Some(-(*n as f64)),
                            _ => None,
                        };
                        if val.is_some() {
                            consumed = 4;
                        }
                    }
                } else {
                    val = match &tokens[current_pos + 2] {
                        Token::Probabilidad(p) => Some(*p),
                        Token::NumeroEntero(n) => Some(*n as f64),
                        _ => None,
                    };
                    if val.is_some() {
                        consumed = 3;
                    }
                }

                if let Some(value) = val {
                    expresiones.push(ExpresionLogica::Comparacion { var: v.clone(), op: op.clone(), val: value });
                    current_pos += consumed;
                } else { break; }
            } else { break; }
        } else { break; }

        if current_pos < tokens.len() {
            if matches!(&tokens[current_pos], Token::YDifuso) {
                operadores.push("y_difuso".to_string());
                current_pos += 1;
            } else if matches!(&tokens[current_pos], Token::ODifuso) {
                operadores.push("o_difuso".to_string());
                current_pos += 1;
            } else { break; }
        } else { break; }
    }

    if expresiones.is_empty() { return Err("Error de sintaxis: Expresión lógica vacía o inválida.".to_string()); }

    let mut expr_tree = expresiones.remove(0);
    for op in operadores {
        if expresiones.is_empty() { return Err("Error de sintaxis en la expresión lógica".to_string()); }
        let next_expr = expresiones.remove(0);
        expr_tree = ExpresionLogica::Logica { expr1: Box::new(expr_tree), op, expr2: Box::new(next_expr) };
    }

    Ok((expr_tree, current_pos))
}