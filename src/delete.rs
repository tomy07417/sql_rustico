use crate::condicion::Condicion;
use crate::my_error::MyError;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

///# Delete
///Esta estructura proporciona toda la funcionalidad necesaria para poder soportar la instrucción
///de DELETE de sql.
///
///**Ejemplo**
///let delete = Delete::new(nombre_archivo, condicion);
///delete.eliminar();
///
///**Parámetros**
///- 'archivo': Contiene la dirección en donde se encuentra el archivo.
///- 'condicion': Contiene la condición que debe cumplir la fila para que sea eliminada o no.
#[derive(Debug, PartialEq)]
pub struct Delete {
    archivo: String,
    condicion: Condicion,
}

impl Delete {
    ///# Delete.new()
    ///Esta función crea una nueva instacia de Delete.
    ///
    ///**Parámetros**
    ///- 'archivo': La dirección de donde se encuentra el archivo al que se quiere aplicar la
    ///  opereción.
    ///- 'condicion': Es la condición que se debe cumplir para que la fila de la tabla sea
    ///  eliminada de la misma.
    ///
    ///**Return**
    ///Devuelve un *struct* de tipo *Delete*.
    pub fn new(archivo: String, condicion: Condicion) -> Self {
        Delete { archivo, condicion }
    }
    ///# Delete.eliminar()
    ///Esta función realiza la eliminación de las filas de una tabla que cumplan la condición que ya tiene
    ///  definida el struct.
    ///
    ///**Return**
    ///Devuelve un *Result<String, MyError>* en caso de que no haya ocurrido un error devuelve el
    ///  *String* para avisar que se realizo la operción y en caso contrario se devuelve un error de
    ///  tipo *MyError*.
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
            .truncate(true)
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

            let verificacion = match self.condicion.verificar(&columnas_tabla, valores) {
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
