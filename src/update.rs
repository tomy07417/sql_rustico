use crate::condicion::Condicion;
use crate::my_error::MyError;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

#[derive(Debug, PartialEq)]
pub struct Update {
    archivo: String,
    valores: Vec<Vec<String>>,
    condicion: Condicion,
}

impl Update {
    pub fn new(archivo: String, valores: Vec<Vec<String>>, condicion: Condicion) -> Self {
        Update {
            archivo,
            valores,
            condicion,
        }
    }

    pub fn update(&self) -> Result<String, MyError> {
        let archivo = match File::open(&self.archivo) {
            Ok(f) => f,
            Err(_e) => {
                return Err(MyError::InvalidTable(
                    "Directorio o nombre de la tabla incorrectos".to_string(),
                ))
            }
        };
        let mut buffer = BufReader::new(archivo);

        let mut archivo_temporal = match OpenOptions::new()
            .write(true)
            .create(true)
            .open("archivo_temporal.csv")
        {
            Ok(f) => f,
            Err(_e) => {
                return Err(MyError::Error(
                    "Fallo en el proceso de modificación de la tabla".to_string(),
                ))
            }
        };

        let mut columnas = String::new();
        match buffer.read_line(&mut columnas) {
            Ok(c) => c,
            Err(_e) => {
                return Err(MyError::Error(
                    "Fallo en el proceso de modificación de la tabla".to_string(),
                ))
            }
        };

        let _ = archivo_temporal.write_all(columnas.as_bytes());

        let columnas_vec: Vec<String> = columnas
            .replace("\n", "")
            .split(',')
            .map(|s| s.to_string())
            .collect();

        for line in buffer.lines() {
            let linea_actual = match line {
                Ok(l) => l + &String::from("\n"),
                Err(_e) => {
                    return Err(MyError::Error(
                        "Fallo en el proceso de edición de la tabla".to_string(),
                    ))
                }
            };

            let valores: &Vec<String> = &linea_actual
                .replace("\n", "")
                .split(',')
                .map(|s| s.to_string())
                .collect();

            let verificacion = match self.condicion.verificar(&columnas_vec, valores) {
                Ok(c) => c,
                Err(e) => return Err(e),
            };

            if verificacion {
                let linea_nueva = self.crear_linea_nueva(valores, &columnas_vec);
                let _ = archivo_temporal.write_all(linea_nueva.as_bytes());
            } else {
                let _ = archivo_temporal.write_all(linea_actual.as_bytes());
            }
        }

        let _ = fs::rename("archivo_temporal.csv", &self.archivo);
        Ok(String::from("Se completo el update correctamente"))
    }

    fn crear_linea_nueva(&self, linea: &Vec<String>, columnas: &Vec<String>) -> String {
        let mut linea_nueva: String = String::new();
        let mut aux: Vec<&String> = Vec::new();

        for i in 0..self.valores.len() {
            let pos: usize = match columnas.iter().position(|x| *x == self.valores[i][0]) {
                Some(i) => i,
                None => 0,
            };
            aux.push(&&columnas[pos]);
        }

        for j in 0..columnas.len() {
            if !linea_nueva.is_empty() {
                linea_nueva.push_str(",");
            }

            if aux.contains(&&columnas[j]) {
                let index = match aux.iter().position(|e| **e == columnas[j]) {
                    Some(i) => i,
                    None => 0,
                };
                linea_nueva.push_str(&self.valores[index][1]);
            } else {
                linea_nueva.push_str(&linea[j]);
            }
        }

        linea_nueva + &String::from("\n")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::condicion_simple::CondicionSimple;

    #[test]
    pub fn test01_se_crea_correctamente_un_update() {
        let operacion = Update::new(
            String::from("./test/update.csv"),
            Vec::<Vec<String>>::new(),
            Condicion::SiempreTrue,
        );

        let operacion_esperada = Update {
            archivo: String::from("./test/update.csv"),
            valores: Vec::<Vec<String>>::new(),
            condicion: Condicion::SiempreTrue,
        };

        assert_eq!(operacion, operacion_esperada);
    }

    #[test]
    pub fn test02_se_realiza_un_update_correctamente() {
        let _ = fs::copy("./test/update_copia.csv", "./test/update.csv");

        let valores = vec![vec![String::from("cantidad"), String::from("4")]];
        let condicion = Condicion::CondicionSimple(CondicionSimple::new(
            "id_cliente".to_string(),
            "=".to_string(),
            "1".to_string(),
        ));
        let operacion = Update::new(String::from("./test/update.csv"), valores, condicion);

        let resultado = operacion.update();

        assert!(resultado.is_ok());
    }
}
