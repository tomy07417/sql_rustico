use crate::condicion::Condicion;
use crate::condicion_simple::CondicionSimple;
use crate::delete::Delete;
use crate::insert::Insert;
use crate::my_error::MyError;
use crate::operacion::Operacion;
use crate::select::Select;
use crate::update::Update;

#[derive(Debug, PartialEq)]
pub struct Parser {
    index: usize,
}

impl Parser {
    pub fn new() -> Self {
        Parser { index: 0 }
    }

    pub fn crear_operacion(
        &mut self,
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

    fn parsear_orden(
        &mut self,
        archivo: String,
        tokens: Vec<String>,
    ) -> Result<Operacion, MyError> {
        return match tokens[self.index] {
           _ if String::from("INSERT") == tokens[self.index] => self.parsear_insert(archivo, tokens),
           _ if String::from("DELETE") == tokens[self.index] => self.parsear_delete(archivo, tokens),
           _ if String::from("UPDATE") == tokens[self.index]  => self.parsear_update(archivo, tokens),
           _ if String::from("SELECT") == tokens[self.index]  => self.parsear_select(archivo, tokens),
            _ => Err(MyError::InvalidSyntax("Instruccion inválida. Las instruccines válidas son: INSERT, DELETE, UPDATE, SELECT".to_string())),
        };
    }
    fn parsear_select(
        &mut self,
        archivo: String,
        tokens: Vec<String>,
    ) -> Result<Operacion, MyError> {
        let mut direccion = archivo;
        let mut columnas: Vec<String> = Vec::new();

        self.avanzar();

        while self.index < tokens.len() && tokens[self.index] != "FROM".to_string() {
            columnas.push(String::from(&tokens[self.index]).replace(",", ""));
            self.avanzar();
        }

        if self.index == tokens.len() || tokens[self.index] != "FROM".to_string() {
            return Err(MyError::InvalidSyntax(
                "Error en la sintaxis de la instrucción (SELECT)".to_string(),
            ));
        }

        self.avanzar();

        let nombre_archivo = "/".to_string() + &String::from(&tokens[self.index]) + ".csv";
        direccion.push_str(&nombre_archivo);

        self.avanzar();

        if self.index < tokens.len() && tokens[self.index] == "WHERE".to_string() {
            self.avanzar();
            let condicion: Condicion = match self.armar_condicion(&tokens, false) {
                Ok(c) => c,
                Err(e) => return Err(e),
            };

            return Ok(Operacion::Select(Select::new(
                direccion, columnas, condicion,
            )));
        } else if self.index == tokens.len() {
            let condicion: Condicion = Condicion::SiempreTrue;
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
        &mut self,
        archivo: String,
        tokens: Vec<String>,
    ) -> Result<Operacion, MyError> {
        self.avanzar();

        let mut direccion = archivo;

        let nombre_archivo = "/".to_string() + &String::from(&tokens[self.index]) + ".csv";
        direccion.push_str(&nombre_archivo);
        self.avanzar();

        if tokens[self.index] != "SET" {
            return Err(MyError::InvalidSyntax(
                "Error en la sintaxis de la instrucción (UPDATE)".to_string(),
            ));
        }

        self.avanzar();
        let valores: Vec<Vec<String>> = match self.armar_valores_update(&tokens) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        if self.index == tokens.len() || tokens[self.index] != "WHERE".to_string() {
            return Err(MyError::InvalidSyntax(
                "Error en la sintaxis de la instrucción (UPDATE)".to_string(),
            ));
        }
        self.avanzar();

        let condicion: Condicion = match self.armar_condicion(&tokens, false) {
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
        &mut self,
        archivo: String,
        tokens: Vec<String>,
    ) -> Result<Operacion, MyError> {
        self.avanzar();

        let mut direccion: String = archivo;

        if tokens[self.index] != "FROM".to_string() {
            return Err(MyError::InvalidSyntax(
                "Error en la sintaxis de la instruccion (DELETE)".to_string(),
            ));
        }
        self.avanzar();

        let nombre_archivo = "/".to_string() + &String::from(&tokens[self.index]) + ".csv";
        direccion.push_str(&nombre_archivo);
        self.avanzar();

        if self.index < tokens.len() && tokens[self.index] == "WHERE".to_string() {
            self.avanzar();
            let condicion: Condicion = match self.armar_condicion(&tokens, false) {
                Ok(c) => c,
                Err(_e) => {
                    return Err(MyError::InvalidSyntax(
                        "Error en la sintaxis de la instrucción (DELETE)".to_string(),
                    ))
                }
            };

            return Ok(Operacion::Delete(Delete::new(direccion, condicion)));
        } else if self.index == tokens.len() {
            let condicion: Condicion = Condicion::SiempreTrue;
            return Ok(Operacion::Delete(Delete::new(direccion, condicion)));
        } else {
            return Err(MyError::InvalidSyntax(
                "Error en la sintaxis de la instrucción (DELETE)".to_string(),
            ));
        }
    }

    fn parsear_insert(
        &mut self,
        archivo: String,
        tokens: Vec<String>,
    ) -> Result<Operacion, MyError> {
        self.avanzar();

        let mut columnas: Vec<String> = Vec::new();
        let mut valores: Vec<Vec<String>> = Vec::new();
        let mut direccion = archivo;

        if tokens[self.index] == "INTO".to_string() {
            self.avanzar();

            let nombre_archivo = "/".to_string() + &String::from(&tokens[self.index]) + ".csv";
            direccion.push_str(&nombre_archivo);
            self.avanzar();

            match self.leer_columnas(&mut columnas, &tokens) {
                Ok(l) => l,
                Err(e) => return Err(e),
            };
            if self.index < tokens.len() && tokens[self.index] == "VALUES".to_string() {
                self.avanzar();

                while self.index < tokens.len() {
                    let mut aux = Vec::new();
                    match self.leer_columnas(&mut aux, &tokens) {
                        Ok(l) => l,
                        Err(e) => return Err(e),
                    };

                    valores.push(aux);
                }
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

        Ok(Operacion::Insert(Insert::new(direccion, columnas, valores)))
    }

    fn avanzar(&mut self) {
        self.index = self.index + 1;
    }

    fn armar_valores_update(&mut self, tokens: &Vec<String>) -> Result<Vec<Vec<String>>, MyError> {
        let mut aux: Vec<Vec<String>> = Vec::<Vec<String>>::new();

        while self.index < tokens.len() && tokens[self.index] != "WHERE".to_string() {
            let clave = String::from(&tokens[self.index]);
            self.avanzar();
            self.avanzar();
            let valor = String::from(&tokens[self.index]).replace(",", "");
            self.avanzar();

            aux.push(vec![clave, valor]);
        }

        if aux.is_empty() {
            return Err(MyError::InvalidSyntax(
                "Error en la sintaxis de la instrucción (UPDATE)".to_string(),
            ));
        }
        Ok(aux)
    }

    fn armar_condicion(
        &mut self,
        tokens: &Vec<String>,
        prioridad: bool,
    ) -> Result<Condicion, MyError> {
        let condiciones = vec!["AND".to_string(), "OR".to_string(), "NOT".to_string()];

        let mut r: Condicion = Condicion::SiempreTrue;
        let mut es_primer_condicion = true;

        while self.index < tokens.len() && !(prioridad && tokens[self.index - 1].contains(")")) {
            if condiciones.contains(&tokens[self.index]) {
                let simb = String::from(&tokens[self.index]);
                self.avanzar();

                let l = match tokens[self.index].contains("(") {
                    true => match self.armar_condicion(tokens, true) {
                        Ok(c) => c,
                        Err(e) => return Err(e),
                    },
                    false => self.armar_condicion_simple(tokens),
                };

                r = match simb {
                    _ if condiciones[0] == simb => Condicion::And(Box::new(r), Box::new(l)),
                    _ if condiciones[1] == simb => Condicion::Or(Box::new(r), Box::new(l)),
                    _ if condiciones[2] == simb => Condicion::Not(Box::new(l)),
                    _ => return Err(MyError::Error("Error inesperado".to_string())),
                }
            } else if es_primer_condicion {
                r = self.armar_condicion_simple(tokens);
                es_primer_condicion = false;
            } else {
                return Err(MyError::InvalidSyntax(
                    "Operador condicional inválido".to_string(),
                ));
            }
        }

        Ok(r)
    }

    fn armar_condicion_simple(&mut self, tokens: &Vec<String>) -> Condicion {
        let col = String::from(&tokens[self.index]).replace("(", "");
        self.avanzar();

        let s = String::from(&tokens[self.index]);
        self.avanzar();

        let val = String::from(&tokens[self.index]).replace(")", "");
        self.avanzar();

        Condicion::CondicionSimple(CondicionSimple::new(col, s, val))
    }

    fn leer_columnas(
        &mut self,
        cols: &mut Vec<String>,
        tokens: &Vec<String>,
    ) -> Result<String, MyError> {
        if tokens[self.index].contains("(") {
            cols.push(String::from(&tokens[self.index].replace("(", "")));
            self.avanzar();

            while !tokens[self.index].contains(")") && self.index < tokens.len() {
                cols.push(String::from(&tokens[self.index]));
                self.avanzar();
            }

            if tokens[self.index].contains(")") {
                cols.push(String::from(&tokens[self.index]).replace(",", ""));
                self.avanzar();
            } else {
                return Err(MyError::InvalidSyntax(
                    "Sintaxis inválida para especificar las columnas".to_string(),
                ));
            }
        } else {
            return Err(MyError::InvalidSyntax(
                "Sintaxis inválida para especificar las columnas".to_string(),
            ));
        }

        Ok(String::from("Se realizo con exito"))
    }
}

#[test]
pub fn test01_se_crea_un_parser_correctamente() {
    let parser = Parser::new();
    let parser_esperado = Parser { index: 0 };

    assert_eq!(parser, parser_esperado);
}

#[test]
pub fn test02a_se_quiere_parsear_un_insert_y_hay_errores_en_la_sintaxis() {
    let mut parser = Parser::new();

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
    let mut parser = Parser::new();

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
    let mut parser = Parser::new();

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
    let mut parser = Parser::new();

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
    let mut parser = Parser::new();

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
    let mut parser = Parser::new();

    let resultado: Result<Operacion, MyError> = parser.crear_operacion(
        "./test".to_string(),
        "UPDATE update SET email = tamundarain@fi.uba.ar id = 3".to_string(),
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
    let mut parser = Parser::new();

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
    let mut parser = Parser::new();

    let resultado: Result<Operacion, MyError> = match parser.crear_operacion(
        "./test".to_string(),
        "INSERT INTO insert (nombre, apellido) VALUES (Tomas, Amundarain)".to_string(),
    ) {
        Ok(l) => Ok(l),
        Err(e) => {
            println!("{}", e);
            Err(e)
        }
    };

    assert!(resultado.is_ok());
}

#[test]
pub fn test06_se_parsea_un_delete_correctamente() {
    let mut parser = Parser::new();

    let resultado: Result<Operacion, MyError> = parser.crear_operacion(
        "./test".to_string(),
        "DELETE FROM insert WHERE nombre = Tomas".to_string(),
    );
    assert!(resultado.is_ok());
}

#[test]
pub fn test07_se_parsea_un_update_correctamente() {
    let mut parser = Parser::new();

    let resultado: Result<Operacion, MyError> = match parser.crear_operacion(
        "./test".to_string(),
        "UPDATE insert SET nombre = Francisco WHERE nombre = Tomas".to_string(),
    ) {
        Ok(c) => Ok(c),
        Err(e) => {
            println!("{}", e);
            Err(e)
        }
    };
    assert!(resultado.is_ok());
}

#[test]
pub fn test08_se_parsea_un_select_correctamente() {
    let mut parser = Parser::new();

    let resultado: Result<Operacion, MyError> = parser.crear_operacion(
        "./test".to_string(),
        "SELECT id FROM select WHERE id_cliente = 1 AND producto = Laptop".to_string(),
    );

    assert!(resultado.is_ok());
}
