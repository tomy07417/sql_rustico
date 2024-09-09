use crate::my_error::MyError;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

#[derive(Debug, PartialEq)]
pub struct Delete {
    archivo: String,
    claves: Vec<String>,
}

impl Delete {
    pub fn new(archivo: String, claves: Vec<String>) -> Self {
        Delete { archivo, claves }
    }

    pub fn eliminar(&self) -> Result<String, MyError> {
        let archivo = match File::open(&self.archivo) {
            Ok(a) => a,
            Err(_e) => {
                return Err(MyError::InvalidTable(
                    "Directorio o nombre de la tabla incorrecto".to_string(),
                ))
            }
        };
        let mut buffer = BufReader::new(archivo);

        let mut archivo_temporal = match OpenOptions::new()
            .write(true)
            .create(true)
            .open("temporal.csv")
        {
            Ok(f) => f,
            Err(_e) => {
                return Err(MyError::Error(
                    "Fallo en el proceso de eliminacion del dato de la tabla".to_string(),
                ))
            }
        };

        let mut linea: String = String::new();
        match buffer.read_line(&mut linea) {
            Ok(l) => l,
            Err(_e) => {
                return Err(MyError::Error(
                    "Fallo en el proceso de eliminación del dato de la tabla".to_string(),
                ))
            }
        };
        let _ = archivo_temporal.write_all(linea.as_bytes());

        let columnas_tabla: Vec<String> = linea
            .replace("\n", "")
            .split(',')
            .map(|s| s.to_string())
            .collect();

        let index_key: usize = match self.obtener_index(&columnas_tabla) {
            Ok(i) => i,
            Err(e) => return Err(e),
        };

        for line in buffer.lines() {
            let linea_actual = match line {
                Ok(l) => l + &String::from("\n"),
                Err(_e) => {
                    return Err(MyError::Error(
                        "Fallo en el proceso de eliminacón del dato de la tabla".to_string(),
                    ))
                }
            };

            let valores: &Vec<String> = &linea_actual.split(',').map(|s| s.to_string()).collect();

            if valores[index_key] != self.claves[1] {
                let _ = archivo_temporal.write_all(linea_actual.as_bytes());
            }
        }

        let _ = fs::rename("temporal.csv", &self.archivo);
        Ok(String::from("Se elimino correctamente el valor"))
    }

    fn obtener_index(&self, columnas_tabla: &Vec<String>) -> Result<usize, MyError> {
        if columnas_tabla.contains(&self.claves[0]) {
            return match columnas_tabla.iter().position(|x| *x == self.claves[0]) {
                Some(i) => Ok(i),
                None => Err(MyError::Error(
                    "El dato que se quiere eliminar no existe".to_string(),
                )),
            };
        } else {
            return Err(MyError::InvalidColumn(
                "La columna que se utilizó como key para buscar el dato no existe".to_string(),
            ));
        }
    }
}

#[test]
pub fn test01_se_crea_un_delete_correctamente() {
    let operacion = Delete::new(String::from("./test/delete.csv"), Vec::<String>::new());

    let operacion_esperada = Delete {
        archivo: String::from("./test/delete.csv"),
        claves: Vec::<String>::new(),
    };

    assert_eq!(operacion, operacion_esperada)
}

#[test]
pub fn test02_se_hace_un_delete_al_archivo_deseado_correctamente() {
    //copio los datos de delete_copia.csv en delete.csv para luego operar en el ultimo
    let _ = fs::copy("./test/delete_copia.csv", "./test/delete.csv");

    let claves = vec![String::from("id"), String::from("5")];
    let operacion = Delete::new(String::from("./test/delete.csv"), claves);

    let resultado = operacion.eliminar();

    assert!(resultado.is_ok())
}

#[test]
pub fn test03_se_hace_un_delete_y_se_quiere_eliminar_un_elemento_con_una_key_que_no_existe() {
    let claves = vec![String::from("id_no_existe"), String::from("5")];
    let operacion = Delete::new(String::from("./test/delete.csv"), claves);

    let resultado = operacion.eliminar();

    assert!(resultado.is_err())
}
