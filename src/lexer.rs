// =====================================================================
// Módulo del Lexer (Analizador Léxico) - Ramati-Q v1.5
// =====================================================================
// Este módulo es responsable de tomar el código fuente como una cadena
// de texto y convertirlo en una secuencia de "Tokens" (fichas léxicas).

use crate::ast::Token;

// Función de mapeo de palabras clave (Keywords)
fn match_keyword(word: &str) -> Token {
    // Se convierte a minúsculas para que las palabras clave no distingan mayúsculas/minúsculas.
    match word.to_lowercase().as_str() {
        "origen" => Token::Origen,
        "colapsar" => Token::Colapsar,
        "usar" => Token::Usar,
        "variable_difusa" => Token::VariableDifusa,
        "sintonizar" => Token::Sintonizar,
        "mostrar" => Token::Mostrar,
        "observar" => Token::Observar,
        "capturar" => Token::Capturar,
        "armonizar" => Token::Armonizar,
        "entrelazar_canales" => Token::EntrelazarCanales,
        "si_difuso" => Token::SiDifuso,
        "sino" => Token::Sino,
        "y_difuso" => Token::YDifuso,
        "o_difuso" => Token::ODifuso,
        "no_difuso" => Token::NoDifuso,
        "bucle_cognitivo" => Token::BucleCognitivo,
        "fin_bucle" => Token::FinBucle,
        "de" => Token::De,
        "accionar" => Token::Accionar,
        "con" => Token::Con,
        "fin_si" => Token::FinSi,
        "mutar" => Token::Mutar,
        "intersecar" => Token::Intersecar,
        "entrelazar" => Token::Entrelazar,
        "resonar_bucle" => Token::ResonarBucle,
        "superponer" => Token::Superponer,
        "fluctuar" => Token::Fluctuar,
        "diagnostico" => Token::Diagnostico,
        "diagnostico_hardware" => Token::DiagnosticoHardware,
        "hasta" => Token::Hasta,
        "entero" => Token::TipoEntero,
        "cadena" => Token::TipoCadena,
        "espectro" => Token::TipoEspectro,
        "funcion" => Token::Func,
        "fin_funcion" => Token::EndFunc,
        "retornar" => Token::Return,
        "regla" => Token::Regla,
        "fin_regla" => Token::FinRegla,
        "objetivo" => Token::Objetivo,
        "conocimiento" => Token::Conocimiento,
        "razonar" => Token::Razonar,
        _ => Token::Identificador(word.to_string()), 
    }
}

fn string_to_token(s: &str) -> Token {
    if let Ok(val) = s.parse::<i64>() {
        Token::NumeroEntero(val)
    } else if let Ok(val) = s.parse::<f64>() {
        Token::Probabilidad(val)
    } else {
        match_keyword(s)
    }
}

pub fn ejecutar_lexer(codigo: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    for linea in codigo.lines() {
        let linea_limpia = linea.split('#').next().unwrap_or("").trim();
        if linea_limpia.is_empty() {
            continue;
        }

        let mut current_line_tokens = Vec::new();
        let mut chars = linea_limpia.chars().peekable();
        let mut current_word = String::new();

        while let Some(&c) = chars.peek() {
            if c.is_whitespace() {
                if !current_word.is_empty() {
                    current_line_tokens.push(string_to_token(&current_word));
                    current_word.clear();
                }
                chars.next(); // Consume whitespace
            } else if c == '"' {
                if !current_word.is_empty() {
                    current_line_tokens.push(string_to_token(&current_word));
                    current_word.clear();
                }
                chars.next(); // Consume opening quote
                let mut string_literal = String::new();
                while let Some(&sc) = chars.peek() {
                    if sc == '"' {
                        chars.next(); // Consume closing quote
                        break;
                    }
                    string_literal.push(chars.next().unwrap());
                }
                current_line_tokens.push(Token::CadenaTexto(string_literal));
            } else if c == '-' && chars.clone().nth(1) == Some('>') { // Handle "->"
                if !current_word.is_empty() {
                    current_line_tokens.push(string_to_token(&current_word));
                    current_word.clear();
                }
                current_line_tokens.push(Token::Flecha);
                chars.next(); // Consume '-'
                chars.next(); // Consume '>'
            } else if c == '(' {
                if !current_word.is_empty() { current_line_tokens.push(string_to_token(&current_word)); current_word.clear(); }
                current_line_tokens.push(Token::ParentesisAbre);
                chars.next();
            } else if c == ')' {
                if !current_word.is_empty() { current_line_tokens.push(string_to_token(&current_word)); current_word.clear(); }
                current_line_tokens.push(Token::ParentesisCierra);
                chars.next();
            } else if c == ',' {
                if !current_word.is_empty() { current_line_tokens.push(string_to_token(&current_word)); current_word.clear(); }
                current_line_tokens.push(Token::Coma);
                chars.next();
            } else if c == '+' {
                if !current_word.is_empty() { current_line_tokens.push(string_to_token(&current_word)); current_word.clear(); }
                current_line_tokens.push(Token::Plus);
                chars.next();
            } else if c == '-' && chars.clone().nth(1) != Some('>') { // Asegurarse de no confundir con '->'
                if !current_word.is_empty() { current_line_tokens.push(string_to_token(&current_word)); current_word.clear(); }
                current_line_tokens.push(Token::Minus);
                chars.next();
            } else if c == '*' {
                if !current_word.is_empty() { current_line_tokens.push(string_to_token(&current_word)); current_word.clear(); }
                current_line_tokens.push(Token::Star);
                chars.next();
            } else if c == '/' {
                if !current_word.is_empty() { current_line_tokens.push(string_to_token(&current_word)); current_word.clear(); }
                current_line_tokens.push(Token::Slash);
                chars.next();
            } else if c == '>' || c == '<' || c == '=' { // Handle relational operators
                if !current_word.is_empty() { current_line_tokens.push(string_to_token(&current_word)); current_word.clear(); }
                let mut op = String::new();
                op.push(chars.next().unwrap()); // Consume the first char (>, <, or =)
                if chars.peek() == Some(&'=') {
                    op.push(chars.next().unwrap()); // Consume '=', forming >=, <=, ==
                }
                current_line_tokens.push(Token::OperadorRelacional(op));
            }
            else {
                current_word.push(chars.next().unwrap());
            }
        }
        if !current_word.is_empty() {
            current_line_tokens.push(string_to_token(&current_word));
        }
        tokens.extend(current_line_tokens);
    }
    tokens
}