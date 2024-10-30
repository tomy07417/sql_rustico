use crate::condicion::Condicion;
use crate::condicion_simple::CondicionSimple;
use crate::delete::Delete;
use crate::insert::Insert;
use crate::my_error::MyError;
use crate::operacion::Operacion;
use crate::select::Select;
use crate::update::Update;

///# Parser
///Esta es la estructura que se encarga de crear las estructuras necesarias para realizar la
///operación deseada al iniciar el programa en base de la instrucción indicada.
///
///**Ejemplo**
///let mut parser = Parser::new();
///let operacion:Operacion = parser.crear_operacion(direccion_archivo, instruccion);
///
///**Parámetros**
///- 'index': Este parámetro es el que se va a utilizar para recorrer la instrucción que se le pase
///  al parser para crear la operación esperada.
#[derive(Debug, PartialEq)]
pub struct Parser {
    index: usize,
}

impl Parser {
    ///# Parser.new()
    ///Esta función crea una nueva instacia de Parser.
    ///
    ///**Return**
    ///Devuelve un *struct* de tipo *Parser*
    pub fn new() -> Self {
        Parser { index: 0 }
    }
    ///# Parser.crear_operacion()
    ///Esta función crea la opreación que representa a la intrucción que se le pasa a la función.
    ///
    ///**Parámetros**
    ///- 'archivo': Es la dirección del archivo que representa a la tabla a la que se quiere
    ///  realizar la instrucción indicada.
    ///- 'instruccion': Es la instrucción que se pasa al comienzo del programa.
    ///
    ///**Return**
    ///Devuelve un *Result<Operacion, MyError>* en caso de que no haya ocurrido ningún error
    ///  durante la ejecución de la función se devuelve un struct de tipo *Operacion* en caso
    ///  contrario se devuelve un error de tipo *MyError*.
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
        match tokens[self.index] {
           _ if *"INSERT" == tokens[self.index] => self.parsear_insert(archivo, tokens),
           _ if *"DELETE" == tokens[self.index] => self.parsear_delete(archivo, tokens),
           _ if *"UPDATE" == tokens[self.index]  => self.parsear_update(archivo, tokens),
           _ if *"SELECT" == tokens[self.index]  => self.parsear_select(archivo, tokens),
            _ => Err(MyError::InvalidSyntax("Instruccion inválida. Las instruccines válidas son: INSERT, DELETE, UPDATE, SELECT".to_string())),
        }
    }
    fn parsear_select(
        &mut self,
        archivo: String,
        tokens: Vec<String>,
    ) -> Result<Operacion, MyError> {
        let mut direccion = archivo;
        let mut columnas: Vec<String> = Vec::new();

        self.avanzar();

        while self.index < tokens.len() && tokens[self.index] != *"FROM" {
            columnas.push(String::from(&tokens[self.index]).replace(",", ""));
            self.avanzar();
        }

        if self.index == tokens.len() || tokens[self.index] != *"FROM" {
            return Err(MyError::InvalidSyntax(
                "Error en la sintaxis de la instrucción (SELECT)".to_string(),
            ));
        }

        self.avanzar();

        let nombre_archivo = "/".to_string() + &String::from(&tokens[self.index]) + ".csv";
        direccion.push_str(&nombre_archivo);

        self.avanzar();
        let mut order: String = String::new();
        let mut asc: bool = true;

        if self.index < tokens.len() && tokens[self.index] == *"WHERE" {
            self.avanzar();
            let condicion: Condicion = match self.armar_condicion(&tokens, false) {
                Ok(c) => c,
                Err(e) => return Err(e),
            };

            if self.index < tokens.len() {
                match self.armar_orden(&tokens, &mut order, &mut asc) {
                    Ok(c) => c,
                    Err(e) => return Err(e),
                };
            }

            Ok(Operacion::Select(Select::new(
                direccion, columnas, condicion, order, asc,
            )))
        } else {
            let condicion: Condicion = Condicion::SiempreTrue;

            if self.index < tokens.len() {
                match self.armar_orden(&tokens, &mut order, &mut asc) {
                    Ok(c) => c,
                    Err(e) => return Err(e),
                };
            }

            Ok(Operacion::Select(Select::new(
                direccion, columnas, condicion, order, asc,
            )))
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

        if self.index == tokens.len() || tokens[self.index] != *"WHERE" {
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

        if tokens[self.index] != *"FROM" {
            return Err(MyError::InvalidSyntax(
                "Error en la sintaxis de la instruccion (DELETE)".to_string(),
            ));
        }
        self.avanzar();

        let nombre_archivo = "/".to_string() + &String::from(&tokens[self.index]) + ".csv";
        direccion.push_str(&nombre_archivo);
        self.avanzar();

        if self.index < tokens.len() && tokens[self.index] == *"WHERE" {
            self.avanzar();
            let condicion: Condicion = match self.armar_condicion(&tokens, false) {
                Ok(c) => c,
                Err(_e) => {
                    return Err(MyError::InvalidSyntax(
                        "Error en la sintaxis de la instrucción (DELETE)".to_string(),
                    ))
                }
            };

            Ok(Operacion::Delete(Delete::new(direccion, condicion)))
        } else if self.index == tokens.len() {
            let condicion: Condicion = Condicion::SiempreTrue;
            Ok(Operacion::Delete(Delete::new(direccion, condicion)))
        } else {
            Err(MyError::InvalidSyntax(
                "Error en la sintaxis de la instrucción (DELETE)".to_string(),
            ))
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

        if tokens[self.index] == *"INTO" {
            self.avanzar();

            let nombre_archivo = "/".to_string() + &String::from(&tokens[self.index]) + ".csv";
            direccion.push_str(&nombre_archivo);
            self.avanzar();

            match self.leer_columnas(&mut columnas, &tokens) {
                Ok(l) => l,
                Err(e) => return Err(e),
            };
            if self.index < tokens.len() && tokens[self.index] == *"VALUES" {
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
        self.index += 1;
    }

    fn armar_orden(
        &mut self,
        tokens: &[String],
        order: &mut String,
        asc: &mut bool,
    ) -> Result<String, MyError> {
        if self.index == tokens.len() || tokens[self.index] != *"ORDER" {
            return Err(MyError::InvalidSyntax(
                "Error de sintaxis al definir el ORDER BY".to_string(),
            ));
        }

        self.avanzar();

        if self.index == tokens.len() || tokens[self.index] != *"BY" {
            return Err(MyError::InvalidSyntax(
                "Error de sintaxis al definir el ORDER BY".to_string(),
            ));
        }

        self.avanzar();

        if self.index == tokens.len() {
            return Err(MyError::InvalidSyntax(
                "Error de sintaxis al definir el ORDER BY".to_string(),
            ));
        }

        *order = String::from(&tokens[self.index]);
        self.avanzar();
        let tipos_de_orden = ["ASC".to_string(), "DESC".to_string()];
        if self.index < tokens.len() && tokens[self.index] == *"DESC" {
            *asc = false;
        } else if self.index < tokens.len() && !tipos_de_orden.contains(&tokens[self.index]) {
            return Err(MyError::InvalidSyntax(
                "Error de sintaxis al definir el ORDER BY".to_string(),
            ));
        }

        Ok("Todo ok".to_string())
    }

    fn armar_valores_update(&mut self, tokens: &[String]) -> Result<Vec<Vec<String>>, MyError> {
        let mut aux: Vec<Vec<String>> = Vec::<Vec<String>>::new();

        while self.index < tokens.len() && tokens[self.index] != *"WHERE" {
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
        let condiciones = ["AND".to_string(), "OR".to_string(), "NOT".to_string()];

        let mut r: Condicion = Condicion::SiempreTrue;
        let mut es_primer_condicion = true;

        while self.index < tokens.len() && !(prioridad && tokens[self.index - 1].contains(")")) {
            if condiciones.contains(&tokens[self.index]) {
                let simb = String::from(&tokens[self.index]);
                self.avanzar();

                if self.index == tokens.len() {
                    return Err(MyError::InvalidSyntax(
                        "Error en la escritura de la condición de la consulta".to_string(),
                    ));
                }

                let l = match tokens[self.index].contains("(") {
                    true => match self.armar_condicion(tokens, true) {
                        Ok(c) => c,
                        Err(e) => return Err(e),
                    },
                    false => {
                        if tokens[self.index] == *"NOT" {
                            self.avanzar();
                            let aux = self.armar_condicion_simple(tokens)?;
                            Condicion::Not(Box::new(aux))
                        } else {
                            self.armar_condicion_simple(tokens)?
                        }
                    }
                };

                r = match simb {
                    _ if condiciones[0] == simb => Condicion::And(Box::new(r), Box::new(l)),
                    _ if condiciones[1] == simb => Condicion::Or(Box::new(r), Box::new(l)),
                    _ if condiciones[2] == simb => Condicion::Not(Box::new(l)),
                    _ => return Err(MyError::Error("Error inesperado".to_string())),
                }
            } else if es_primer_condicion {
                r = self.armar_condicion_simple(tokens)?;
                es_primer_condicion = false;
            } else {
                return Err(MyError::InvalidSyntax(
                    "Operador condicional inválido".to_string(),
                ));
            }
        }

        Ok(r)
    }

    fn armar_condicion_simple(&mut self, tokens: &[String]) -> Result<Condicion, MyError> {
        let col = String::from(&tokens[self.index]).replace("(", "");
        self.avanzar();

        if self.index == tokens.len() {
            return Err(MyError::InvalidSyntax(
                "Error en la escritura de la condición de la consulta".to_string(),
            ));
        }

        let s = String::from(&tokens[self.index]);
        self.avanzar();

        if self.index == tokens.len() {
            return Err(MyError::InvalidSyntax(
                "Error en la escritura de la condición de la consulta".to_string(),
            ));
        }

        let val = String::from(&tokens[self.index]).replace(")", "");
        self.avanzar();

        Ok(Condicion::CondicionSimple(CondicionSimple::new(
            col, s, val,
        )))
    }

    fn leer_columnas(
        &mut self,
        cols: &mut Vec<String>,
        tokens: &[String],
    ) -> Result<String, MyError> {
        if tokens[self.index].contains("(") && tokens[self.index].contains(")") {
            cols.push(String::from(
                &tokens[self.index]
                    .replace("(", "")
                    .replace('"', "")
                    .replace(")", ""),
            ));
            self.avanzar();
            return Ok("Ok".to_string());
        }

        if tokens[self.index].contains("(") {
            cols.push(String::from(
                &tokens[self.index].replace("(", "").replace('"', ""),
            ));
            self.avanzar();

            while !tokens[self.index].contains(")") && self.index < tokens.len() {
                cols.push(String::from(&tokens[self.index]));
                self.avanzar();
            }

            if tokens[self.index].contains(")") {
                cols.push(
                    String::from(&tokens[self.index])
                        .replace(")", "")
                        .replace(",", "")
                        .replace('"', ""),
                );
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

impl Default for Parser {
    fn default() -> Self {
        Self::new()
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
