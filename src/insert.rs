use crate::my_error::MyError;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Write};

#[derive(Debug, PartialEq)]
pub struct Insert {
    archivo: String,
    columnas: Vec<String>,
    valores: Vec<String>,
}

impl Insert {
    pub fn new(archivo: String, columnas: Vec<String>, valores: Vec<String>) -> Self {
        Insert {
            archivo,
            columnas,
            valores,
        }
    }

    pub fn insertar(&self) -> Result<String, MyError> {
        let archivo = match File::open(&self.archivo) {
            Ok(f) => f,
            Err(_e) => {
                return Err(MyError::InvalidTable(
                    "Directorio o nombre de la tabla incorrecto".to_string(),
                ))
            }
        };

        let mut buffer = io::BufReader::new(archivo);

        let mut linea = String::new();
        match buffer.read_line(&mut linea) {
            Ok(l) => l,
            Err(_e) => {
                return Err(MyError::Error(String::from(
                    "Fallo en la lectura de la tabla",
                )))
            }
        };

        let columnas_tablas: Vec<String> = linea
            .replace("\n", "")
            .split(',')
            .map(|s| s.to_string())
            .collect();

        if !(columnas_tablas == self.columnas) {
            return Err(MyError::InvalidColumn(
                "Las columnas ingresadas no son válidas para la tabla que se quiere modificar"
                    .to_string(),
            ));
        };

        let mut archivo_escritura = match OpenOptions::new().append(true).open(&self.archivo) {
            Ok(a) => a,
            Err(_e) => {
                return Err(MyError::Error(String::from(
                    "Fallo en la edición de la tabla",
                )))
            }
        };

        let mut linea_nueva = String::new();
        for valor in &self.valores {
            if linea_nueva.is_empty() {
                linea_nueva = linea_nueva + valor;
            } else {
                linea_nueva = linea_nueva + &String::from(',') + valor;
            }
        }

        linea_nueva = linea_nueva + "\n";

        let _ = archivo_escritura.write_all(linea_nueva.as_bytes());

        Ok(String::from("Insert exitoso"))
    }
}

#[test]
pub fn test01_se_crea_un_insert_correctamente() {
    let operacion = Insert::new(String::from("~/test/insert.csv"), Vec::new(), Vec::new());

    let operacion_esperada = Insert {
        archivo: String::from("~/test/insert.csv"),
        columnas: Vec::new(),
        valores: Vec::new(),
    };

    assert_eq!(operacion, operacion_esperada);
}

#[test]
pub fn test02_hace_un_insert_correctamente_al_archivo_deseado() {
    let columnas: Vec<String> = vec![String::from("nombre"), String::from("apellido")];
    let valores: Vec<String> = vec![String::from("Tomas"), String::from("Amundarain")];
    let operacion = Insert::new(String::from("./test/insert.csv"), columnas, valores);

    let resultado = operacion.insertar();

    assert!(resultado.is_ok())
}

#[test]
pub fn test03_se_hace_un_insert_y_salta_un_error_por_columna_invalida() {
    let columnas: Vec<String> = vec![
        String::from("nombre"),
        String::from("apellido"),
        String::from("columna_extra"),
    ];
    let valores: Vec<String> = vec![String::from("Tomas"), String::from("Amundarain")];
    let operacion = Insert::new(String::from("./test/insert.csv"), columnas, valores);

    let resultado = operacion.insertar();
    let _descripcion_error = String::from(
        "Las columnas ingresadas no son válidas para la tabla que se quiere modificar",
    );

    assert!(resultado.is_err());
    assert!(matches!(
        resultado.unwrap_err(),
        MyError::InvalidColumn(_descripcion_error)
    ));
}

#[test]
pub fn test04_se_hace_un_insert_y_salta_un_error_por_tabla_invalida() {
    let columnas: Vec<String> = vec![
        String::from("nombre"),
        String::from("apellido"),
        String::from("columna_extra"),
    ];
    let valores: Vec<String> = vec![String::from("Tomas"), String::from("Amundarain")];
    let operacion = Insert::new(String::from("./test/isert.csv"), columnas, valores);

    let resultado = operacion.insertar();
    let _descripcion_error = String::from("Directorio o el nombre de la tabla incorrecto ");

    assert!(resultado.is_err());
    assert!(matches!(
        resultado.unwrap_err(),
        MyError::InvalidTable(_descripcion_error)
    ));
}
