// =====================================================================
// Módulo del Árbol de Sintaxis Abstracto (AST) - Ramati-Q v1.5
// =====================================================================
// Este módulo define las estructuras de datos que representan el código
// Ramati-Q una vez que ha sido parseado: Tokens, Expresiones y Nodos.

use crate::entorno::ValorRamati;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Origen,
    Colapsar,
    Usar,
    VariableDifusa,
    Sintonizar,
    Mostrar,
    Observar,
    Capturar,
    Armonizar,
    EntrelazarCanales,
    SiDifuso,
    Sino,
    YDifuso,
    ODifuso,
    NoDifuso,
    Mutar,
    Accionar, // 'accionar'
    Con,      // 'con'
    FinSi,    // 'fin_si'
    Intersecar,
    BucleCognitivo, // 'bucle_cognitivo'
    FinBucle,       // 'fin_bucle'
    De,             // 'de'
    Entrelazar,
    ResonarBucle,
    Superponer,
    Fluctuar,
    Diagnostico,
    DiagnosticoHardware,
    Identificador(String),
    CadenaTexto(String),
    Probabilidad(f64),
    NumeroEntero(i64),
    OperadorRelacional(String),
    Hasta,
    Flecha,
    ParentesisAbre,
    ParentesisCierra,
    Coma,
    Plus,   // +
    Minus,  // -
    Star,   // *
    Slash,  // /
    Func,         // 'funcion'
    EndFunc,      // 'fin_funcion'
    Return,       // 'retornar'
    Regla,        // 'regla'
    FinRegla,     // 'fin_regla'
    Objetivo,     // 'objetivo'
    Conocimiento, // 'conocimiento'
    Razonar,      // 'razonar'
    TipoEspectro,
    TipoCadena,
    TipoEntero,
}

#[derive(Debug, Clone)]
pub enum ExpresionLogica {
    Comparacion { var: String, op: String, val: f64 },
    Logica { expr1: Box<ExpresionLogica>, op: String, expr2: Box<ExpresionLogica> },
}

#[derive(Debug, Clone)]
pub enum Expresion {
    Literal(ValorRamati),
    Variable(String),
    Binaria {
        izquierda: Box<Expresion>,
        operador: Token,
        derecha: Box<Expresion>,
    },
    Agrupacion { expresion: Box<Expresion> },
    LlamadaFuncion {
        nombre: String,
        argumentos: Vec<Expresion>,
    },
}

#[derive(Debug, Clone)]
pub enum Nodo {
    Programa(Vec<Nodo>),
    UsarModulo(String),
    ExpresionStatement { expresion: Expresion },
    DeclaracionCubit { nombre: String, probabilidad: f64 },
    SintonizacionCubit { nombre: String, probabilidad: f64 },
    ComandoMostrar(Expresion),
    ObservarCubit(String),
    CapturarDato { variable: String, canal: String },
    ArmonizarSistema,
    EntrelazarCanalesFisicos(String),
    MutarSistema,
    DiagnosticoSistema,
    FluctuarCubit(String),
    SuperponerCubit(String),
    IntersecarVariables { destino: String, var1: String, var2: String },
    EntrelazarVariables { var1: String, var2: String },
    ResonarBucleSistema { var1: String, var2: String, delta: f64 },
    CondicionalDifuso { expresion: ExpresionLogica, cuerpo_si: Box<Nodo>, cuerpo_sino: Option<Box<Nodo>> },
    DeclaracionEntero { nombre: String, valor: Expresion },
    DeclaracionCadena { nombre: String, valor: Expresion },
    DeclaracionEspectro { nombre: String, valor: Expresion },
    DeclaracionFuncion { nombre: String, parametros: Vec<String>, cuerpo: Box<Nodo> },
    Retornar { valor: Expresion },
    DeclaracionRegla { nombre: String, cuerpo: ExpresionLogica },
    BucleCognitivo { ciclos: Expresion, cuerpo: Box<Nodo> },
    ComandoObjetivo { x: Expresion, y: Expresion },
    ComandoRazonar,
    ComandoAccionar { actuador: String, valor: Expresion },
    DiagnosticoHardware,
}