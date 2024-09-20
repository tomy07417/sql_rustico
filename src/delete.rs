use crate::condicion::Condicion;
use crate::my_error::MyError;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

#[derive(Debug, PartialEq)]
pub struct Delete {
    archivo: String,
    condicion: Condicion,
}

impl Delete {
    pub fn new(archivo: String, condicion: Condicion) -> Self {
        Delete { archivo, condicion }
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

            let verificacion = match self.condicion.verificar(&columnas_tabla, &valores) {
                Ok(c) => c,
                Err(e) => return Err(e),
            };

            if !verificacion {
                let _ = archivo_temporal.write_all(linea_actual.as_bytes());
            }
        }

        let _ = fs::rename("temporal.csv", &self.archivo);
        Ok(String::from("Se elimino correctamente el valor"))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::condicion_simple::CondicionSimple;

    #[test]
    pub fn test01_se_crea_un_delete_correctamente() {
        let operacion = Delete::new(String::from("./test/delete.csv"), Condicion::SiempreTrue);

        let operacion_esperada = Delete {
            archivo: String::from("./test/delete.csv"),
            condicion: Condicion::SiempreTrue,
        };

        assert_eq!(operacion, operacion_esperada)
    }

    #[test]
    pub fn test02_se_hace_un_delete_al_archivo_deseado_correctamente() {
        //copio los datos de delete_copia.csv en delete.csv para luego operar en el ultimo
        let _ = fs::copy("./test/delete_copia.csv", "./test/delete.csv");

        let clave = Condicion::CondicionSimple(CondicionSimple::new(
            "id".to_string(),
            "=".to_string(),
            "5".to_string(),
        ));
        let operacion = Delete::new(String::from("./test/delete.csv"), clave);

        let resultado = operacion.eliminar();

        assert!(resultado.is_ok())
    }

    #[test]
    pub fn test03_se_hace_un_delete_y_se_quiere_eliminar_un_elemento_con_una_key_que_no_existe() {
        let claves = Condicion::CondicionSimple(CondicionSimple::new(
            "key_no_existe".to_string(),
            "=".to_string(),
            "9".to_string(),
        ));
        let operacion = Delete::new(String::from("./test/delete.csv"), claves);

        let resultado = operacion.eliminar();

        assert!(resultado.is_err())
    }
}
