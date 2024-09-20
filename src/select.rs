use crate::condicion::Condicion;
use crate::my_error::MyError;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq)]
pub struct Select {
    archivo: String,
    columnas: Vec<String>,
    condicion: Condicion,
}

impl Select {
    pub fn new(archivo: String, columnas: Vec<String>, condicion: Condicion) -> Self {
        Select {
            archivo,
            columnas,
            condicion,
        }
    }

    pub fn seleccionar(&self) -> Result<String, MyError> {
        let archivo = match File::open(&self.archivo) {
            Ok(f) => f,
            Err(_e) => {
                return Err(MyError::InvalidTable(
                    "Directorio o nombre de la tabla incorrecto".to_string(),
                ))
            }
        };

        let mut buffer = BufReader::new(archivo);

        let mut linea = String::new();
        match buffer.read_line(&mut linea) {
            Ok(l) => l,
            Err(_e) => {
                return Err(MyError::Error(
                    "Fallo en el proceso de selección".to_string(),
                ))
            }
        };

        let columnas = linea
            .replace("\n", "")
            .split(",")
            .map(|s| s.to_string())
            .collect();

        let _ = match self.corroborar_columnas(&columnas) {
            Ok(i) => i,
            Err(e) => return Err(e),
        };

        let mut lineas_elegidas: Vec<Vec<String>> = Vec::<Vec<String>>::new();

        for line in buffer.lines() {
            let linea = match line {
                Ok(l) => l,
                Err(_e) => {
                    return Err(MyError::Error(
                        "Fallo en el proceso de selección".to_string(),
                    ))
                }
            };

            let datos: Vec<String> = linea
                .replace("\n", "")
                .split(",")
                .map(|s| s.to_string())
                .collect();

            let verificacion = match self.condicion.verificar(&columnas, &datos) {
                Ok(c) => c,
                Err(e) => return Err(e),
            };

            if verificacion {
                lineas_elegidas.push(datos);
            }
        }

        let mut aux = String::new();
        for c in &self.columnas {
            if !aux.is_empty() {
                aux.push_str(",");
            }

            aux.push_str(c);
        }
        self.mostrar_lineas_elegidas(lineas_elegidas, columnas);

        Ok("Proceso completo".to_string())
    }

    fn mostrar_lineas_elegidas(&self, lineas: Vec<Vec<String>>, col: Vec<String>) {
        for l in &lineas {
            let mut aux = String::new();

            for c in &self.columnas {
                let pos = match col.iter().position(|d| *d == *c) {
                    Some(d) => d,
                    None => 0,
                };

                if !aux.is_empty() {
                    aux.push_str(",");
                }

                aux.push_str(&l[pos]);
            }

            println!("{}", aux);
        }
    }

    fn corroborar_columnas(&self, columnas: &Vec<String>) -> Result<String, MyError> {
        for col in &self.columnas {
            if !columnas.contains(&col) {
                return Err(MyError::InvalidColumn(
                    "Hay columnas en la instrucción que no existen en la tabla".to_string(),
                ));
            }
        }

        Ok("Todo correcto".to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::condicion_simple::CondicionSimple;

    #[test]
    pub fn test01_se_crea_un_select_correctamente() {
        let select = Select::new(
            "./test/select.rs".to_string(),
            Vec::<String>::new(),
            Condicion::SiempreTrue,
        );

        let select_esperado = Select {
            archivo: "./test/select.rs".to_string(),
            columnas: Vec::<String>::new(),
            condicion: Condicion::SiempreTrue,
        };

        assert_eq!(select_esperado, select);
    }

    #[test]
    pub fn test02_se_realiza_un_select_correctamente() {
        let columnas = vec![
            "id_cliente".to_string(),
            "producto".to_string(),
            "cantidad".to_string(),
        ];
        let condicion = Condicion::CondicionSimple(CondicionSimple::new(
            "id_cliente".to_string(),
            "=".to_string(),
            "1".to_string(),
        ));
        let select = Select::new("./test/select.csv".to_string(), columnas, condicion);

        let resultado = select.seleccionar();

        assert!(resultado.is_ok());
    }
}
