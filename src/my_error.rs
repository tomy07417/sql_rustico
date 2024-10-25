use std::fmt;

///# MyError
///Esta estructura proporciona los tipos de errores que se manejarán en la ejecución del programa.
///
///**Tipos**
///- *InvalidTable*: Son los errores relacionados a la tabla.
///- *InvalidColumn*: Son los errores relacionados a las columnas de la tabla.
///- *InvalidSyntax*: Son los errores relacionados a la sintaxis de las instrucciones escritas.
///- *Error*: Son los todos los otros tipos de errores que ocurren que no estan relacionados a los
///  antes mencionados.
#[derive(Debug)]
pub enum MyError {
    InvalidTable(String),
    InvalidColumn(String),
    InvalidSyntax(String),
    Error(String),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::InvalidTable(ref msg) => write!(f, "INVALID_TABLE: {}", msg),
            MyError::InvalidColumn(ref msg) => write!(f, "INVALID_COLUMN: {}", msg),
            MyError::InvalidSyntax(ref msg) => write!(f, "INVALID_SYNTAX: {}", msg),
            MyError::Error(ref msg) => write!(f, "ERROR: {}", msg),
        }
    }
}
