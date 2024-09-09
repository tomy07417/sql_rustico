use crate::delete::Delete;
use crate::insert::Insert;
use crate::my_error::MyError;
use crate::operacion::Operacion;
use crate::select::Select;
use crate::update::Update;

#[derive(Debug, PartialEq)]
pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
        Parser {}
    }

    pub fn crear_operacion(
        &self,
        archivo: String,
        instruccion: String,
    ) -> Result<Operacion, MyError> {
        let tokens: Vec<String> = instruccion
            .replace(",", "")
            .split(' ')
            .map(|s| s.to_string())
            .collect();
        self.parsear_orden(archivo, tokens)
    }

    fn parsear_orden(&self, archivo: String, tokens: Vec<String>) -> Result<Operacion, MyError> {
        return match tokens[0] {
           _ if String::from("INSERT") == tokens[0] => self.parsear_insert(archivo, tokens),
           _ if String::from("DELETE") == tokens[0] => self.parsear_delete(archivo, tokens),
           _ if String::from("UPDATE") == tokens[0]  => self.parsear_update(archivo, tokens),
           _ if String::from("SELECT") == tokens[0]  => self.parsear_select(archivo, tokens),
            _ => Err(MyError::InvalidSyntax("Instruccion inválida. Las instruccines válidas son: INSERT, DELETE, UPDATE, SELECT".to_string())),
        };
    }
    fn parsear_select(
        &self,
        archivo: String,
        mut tokens: Vec<String>,
    ) -> Result<Operacion, MyError> {
        let mut direccion = archivo;
        let mut columnas: Vec<String> = Vec::new();

        tokens.remove(0);

        while !tokens.is_empty() && tokens[0] != "FROM".to_string() {
            columnas.push(tokens.remove(0).replace(",", ""));
        }

        if tokens.is_empty() || tokens[0] != "FROM".to_string() {
            return Err(MyError::InvalidSyntax(
                "Error en la sintaxis de la instrucción (SELECT)".to_string(),
            ));
        }

        tokens.remove(0);

        let nombre_archivo = "/".to_string() + &tokens.remove(0) + ".csv";
        direccion.push_str(&nombre_archivo);

        if !tokens.is_empty() && tokens[0] == "WHERE".to_string() {
            tokens.remove(0);
            let condicion: Vec<String> =
                tokens.remove(0).split("=").map(|s| s.to_string()).collect();
            return Ok(Operacion::Select(Select::new(
                direccion, columnas, condicion,
            )));
        } else {
            return Err(MyError::InvalidSyntax(
                "Error en la sintaxis de la instrucción (SELECT)".to_string(),
            ));
        }
    }
    fn parsear_update(
        &self,
        archivo: String,
        mut tokens: Vec<String>,
    ) -> Result<Operacion, MyError> {
        tokens.remove(0);

        let mut direccion = archivo;

        let nombre_archivo = "/".to_string() + &tokens.remove(0) + ".csv";
        direccion.push_str(&nombre_archivo);

        if tokens[0] != "SET" {
            return Err(MyError::InvalidSyntax(
                "Error en la sintaxis de la instrucción (UPDATE)".to_string(),
            ));
        }
        tokens.remove(0);
        let valores: Vec<Vec<String>> = match self.armar_valores_update(&mut tokens) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        if tokens.is_empty() || tokens[0] != "WHERE".to_string() {
            return Err(MyError::InvalidSyntax(
                "Error en la sintaxis de la instrucción (UPDATE)".to_string(),
            ));
        }

        let condicion: Vec<String> = match self.armar_condicion(&mut tokens) {
            Ok(c) => c,
            Err(_e) => {
                return Err(MyError::InvalidSyntax(
                    "Error en la sintaxis de la instrucción (UPDATE)".to_string(),
                ))
            }
        };
        Ok(Operacion::Update(Update::new(
            direccion, valores, condicion,
        )))
    }

    fn parsear_delete(
        &self,
        archivo: String,
        mut tokens: Vec<String>,
    ) -> Result<Operacion, MyError> {
        tokens.remove(0);

        let mut direccion: String = archivo;

        if tokens[0] != "FROM".to_string() {
            return Err(MyError::InvalidSyntax(
                "Error en la sintaxis de la instruccion (DELETE)".to_string(),
            ));
        }
        tokens.remove(0);
        let nombre_archivo = "/".to_string() + &tokens.remove(0) + ".csv";
        direccion.push_str(&nombre_archivo);

        if !tokens.is_empty() && tokens[0] == "WHERE".to_string() {
            let condicion: Vec<String> = match self.armar_condicion(&mut tokens) {
                Ok(c) => c,
                Err(_e) => {
                    return Err(MyError::InvalidSyntax(
                        "Error en la sintaxis de la instrucción (DELETE)".to_string(),
                    ))
                }
            };

            return Ok(Operacion::Delete(Delete::new(direccion, condicion)));
        } else {
            return Err(MyError::InvalidSyntax(
                "Error en la sintaxis de la instrucción (DELETE)".to_string(),
            ));
        }
    }

    fn parsear_insert(
        &self,
        archivo: String,
        mut tokens: Vec<String>,
    ) -> Result<Operacion, MyError> {
        tokens.remove(0);
        let mut columnas: Vec<String> = Vec::new();
        let mut valores: Vec<String> = Vec::new();
        let mut direccion = archivo;

        if tokens[0] == "INTO".to_string() {
            tokens.remove(0);
            let nombre_archivo = "/".to_string() + &tokens.remove(0) + ".csv";
            direccion.push_str(&nombre_archivo);

            match self.leer_columnas(&mut columnas, &mut tokens) {
                Ok(l) => l,
                Err(e) => return Err(e),
            };
            if !tokens.is_empty() && tokens[0] == "VALUES".to_string() {
                tokens.remove(0);
                let _ = match self.leer_columnas(&mut valores, &mut tokens) {
                    Ok(l) => l,
                    Err(e) => return Err(e),
                };
            } else {
                return Err(MyError::InvalidSyntax(
                    "Error en la sintaxis de la instrucción (INSERT)".to_string(),
                ));
            }
        } else {
            return Err(MyError::InvalidSyntax(
                "Error en la sintaxis de la intrucción (INSERT)".to_string(),
            ));
        }

        println!("{} {:?} {:?}", direccion, columnas, valores);
        Ok(Operacion::Insert(Insert::new(direccion, columnas, valores)))
    }

    fn armar_valores_update(&self, tokens: &mut Vec<String>) -> Result<Vec<Vec<String>>, MyError> {
        let mut aux: Vec<Vec<String>> = Vec::<Vec<String>>::new();

        while !tokens.is_empty() && tokens[0] != "WHERE".to_string() {
            aux.push(tokens.remove(0).split('=').map(|s| s.to_string()).collect());
        }

        if aux.is_empty() {
            return Err(MyError::InvalidSyntax(
                "Error en la sintaxis de la instrucción (UPDATE)".to_string(),
            ));
        }

        Ok(aux)
    }

    fn armar_condicion(&self, tokens: &mut Vec<String>) -> Result<Vec<String>, MyError> {
        let mut aux: Vec<String> = Vec::new();

        if tokens.is_empty() {
            return Err(MyError::InvalidSyntax(
                "Error de sintaxis de la instrucción".to_string(),
            ));
        }

        while !tokens.is_empty() {
            aux = tokens.remove(0).split("=").map(|s| s.to_string()).collect();
        }

        Ok(aux)
    }

    fn leer_columnas(
        &self,
        cols: &mut Vec<String>,
        tokens: &mut Vec<String>,
    ) -> Result<String, MyError> {
        if tokens[0].contains("(") {
            cols.push(tokens.remove(0).replace("(", ""));

            while !tokens[0].contains(")") && !tokens.is_empty() {
                cols.push(tokens.remove(0));
            }

            cols.push(tokens.remove(0).replace(")", ""));
        }

        Ok(String::from("Se realizo con exito"))
    }
}

#[test]
pub fn test01_se_crea_un_parser_correctamente() {
    let parser = Parser::new();
    let parser_esperado = Parser {};

    assert_eq!(parser, parser_esperado);
}

#[test]
pub fn test02a_se_quiere_parsear_un_insert_y_hay_errores_en_la_sintaxis() {
    let parser = Parser::new();

    let resultado: Result<Operacion, MyError> = parser.crear_operacion(
        "./test".to_string(),
        "INSERT INTO insert (nombre, apellido) (Tomas, Amundarain)".to_string(),
    );
    let _msg = "Error en la sintaxis de la instrucción (INSERT)".to_string();

    assert!(resultado.is_err());
    assert!(matches!(
        resultado.unwrap_err(),
        MyError::InvalidSyntax(_msg)
    ));
}

#[test]
pub fn test02b_se_quiere_parsear_un_insert_y_hay_errores_en_la_sintaxis() {
    let parser = Parser::new();

    let resultado: Result<Operacion, MyError> = parser.crear_operacion(
        "./test".to_string(),
        "INSERT insert (nombre, apellido) VALUE (Tomas, Amundarain)".to_string(),
    );
    let _msg = "Error en la sintaxis de la instrucción (INSERT)".to_string();

    assert!(resultado.is_err());
    assert!(matches!(
        resultado.unwrap_err(),
        MyError::InvalidSyntax(_msg)
    ));
}

#[test]
pub fn test02c_se_quiere_parsear_un_insert_y_hay_errores_en_la_sintaxis() {
    let parser = Parser::new();

    let resultado: Result<Operacion, MyError> = parser.crear_operacion(
        "./test".to_string(),
        "INSERT INTO (nombre, apellido) VALUE (Tomas, Amundarain)".to_string(),
    );
    let _msg = "Error en la sintaxis de la instrucción (INSERT)".to_string();

    assert!(resultado.is_err());
    assert!(matches!(
        resultado.unwrap_err(),
        MyError::InvalidSyntax(_msg)
    ));
}

#[test]
pub fn test03a_se_quiere_parsear_un_delete_y_hay_errores_en_la_sintaxis() {
    let parser = Parser::new();

    let resultado: Result<Operacion, MyError> =
        parser.crear_operacion("./test".to_string(), "DELETE FROM delete id".to_string());
    let _msg = "Error en la sintaxis de la instrucción (DELETE)".to_string();

    assert!(resultado.is_err());
    assert!(matches!(
        resultado.unwrap_err(),
        MyError::InvalidSyntax(_msg)
    ));
}

#[test]
pub fn test03b_se_quiere_parsear_un_delete_y_hay_errores_en_la_sintaxis() {
    let parser = Parser::new();

    let resultado: Result<Operacion, MyError> =
        parser.crear_operacion("./test".to_string(), "DELETE delete WHERE id".to_string());
    let _msg = "Error en la sintaxis de la instrucción (DELETE)".to_string();

    assert!(resultado.is_err());
    assert!(matches!(
        resultado.unwrap_err(),
        MyError::InvalidSyntax(_msg)
    ));
}

#[test]
pub fn test04a_se_quiere_parsear_un_update_y_hay_errores_en_la_sintaxis() {
    let parser = Parser::new();

    let resultado: Result<Operacion, MyError> = parser.crear_operacion(
        "./test".to_string(),
        "UPDATE update SET email=tamundarain@fi.uba.ar id=3".to_string(),
    );
    let _msg = "Error en la sintaxis de la instrucción (UPDATE)".to_string();

    assert!(resultado.is_err());
    assert!(matches!(
        resultado.unwrap_err(),
        MyError::InvalidSyntax(_msg)
    ));
}

#[test]
pub fn test04b_se_quiere_parsear_un_update_y_hay_errores_en_la_sintaxis() {
    let parser = Parser::new();

    let resultado: Result<Operacion, MyError> = parser.crear_operacion(
        "./test".to_string(),
        "UPDATE update SET DONDE email=tamundarain@fi.uba.ar id=3".to_string(),
    );
    let _msg = "Error en la sintaxis de la instrucción (UPDATE)".to_string();

    assert!(resultado.is_err());
    assert!(matches!(
        resultado.unwrap_err(),
        MyError::InvalidSyntax(_msg)
    ));
}

#[test]
pub fn test05_se_parsea_un_insert_correctamente() {
    let parser = Parser::new();

    let resultado: Result<Operacion, MyError> = parser.crear_operacion(
        "./test".to_string(),
        "INSERT INTO insert (nombre, apellido) VALUES (Tomas, Amundarain)".to_string(),
    );
    assert!(resultado.is_ok());
}

#[test]
pub fn test06_se_parsea_un_delete_correctamente() {
    let parser = Parser::new();

    let resultado: Result<Operacion, MyError> = parser.crear_operacion(
        "./test".to_string(),
        "DELETE FROM insert WHERE nombre=Tomas".to_string(),
    );
    assert!(resultado.is_ok());
}

#[test]
pub fn test07_se_parsea_un_update_correctamente() {
    let parser = Parser::new();

    let resultado: Result<Operacion, MyError> = parser.crear_operacion(
        "./test".to_string(),
        "UPDATE insert SET nombre=Francisco WHERE nombre=Tomas".to_string(),
    );
    assert!(resultado.is_ok());
}
