use crate::condicion::Condicion;
use crate::my_error::MyError;
use std::fs::File;
use std::io::{BufRead, BufReader};

///# Select
///Esta estructura proporciona todo lo necesario para la implementación de la instrucción SELECT en
///sql.
///
///**Ejemplo**
///let select = Select::new(direccion_archivo, columnas, condicion, nombre_columna_order, asc);
///select.seleccionar();
///
///**Parámetros**
///- 'archivo': Es a la dirección del archivo que representa a la tabla que se le quiere
///realizar dicha operación.
///- 'columnas': Es un array que tiene el nombre de las columnas que se quieren imprimir por
///pantalla (No hace falta que tengan el orden que tienen en la tabla).
///- 'condicion': Es la condición que deben cumplir la fila de la tabla para que puedan ser
///imprimidas por pantalla.
///- 'order': Contiene el nombre de la columna por la que se tienen que ordenar las filas que se
///tienen que mostrar en caso de pedirlo.
///- 'asc': Es el valor que representa, en caso de pedirlo, a ordenar de manera ascendente si es
///true o descendente si es false.
#[derive(Debug, PartialEq)]
pub struct Select {
    archivo: String,
    columnas: Vec<String>,
    condicion: Condicion,
    order: String,
    asc: bool,
}

impl Select {
    ///# Select.new()
    ///Esta función crea una nueva instacia de Select.
    ///
    ///**Parámetros**
    ///- 'archivo': Es a la dirección del archivo que representa a la tabla que se le quiere
    ///realizar dicha operación.
    ///- 'columnas': Es un array que tiene el nombre de las columnas que se quieren imprimir por
    ///pantalla (No hace falta que tengan el orden que tienen en la tabla).
    ///- 'condicion': Es la condición que deben cumplir la fila de la tabla para que puedan ser
    ///imprimidas por pantalla.
    ///- 'order': Contiene el nombre de la columna por la que se tienen que ordenar las filas que se
    ///tienen que mostrar en caso de pedirlo.
    ///- 'asc': Es el valor que representa, en caso de pedirlo, a ordenar de manera ascendente si es
    ///true o descendente si es false.
    ///
    ///**Return**
    ///Devuelve un *struct* del tipo *Select*.
    pub fn new(
        archivo: String,
        columnas: Vec<String>,
        condicion: Condicion,
        order: String,
        asc: bool,
    ) -> Self {
        Select {
            archivo,
            columnas,
            condicion,
            order,
            asc,
        }
    }
    ///# Selcet.seleccionar()
    ///Esta función realiza la instrucción SELECT de sql.
    ///
    ///**Return**
    ///Devuelve un *Result<String, MyError>* en caso que durante la ejecución de la función no haya
    ///ocurrido ningún erro se devuelve un *String* para indicar que la opreción se realizó
    ///correctamente, en caso contrario se retorna un error del tipo *MyError*.
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
        let _ = self.ordenar_lineas_elegidas(&mut lineas_elegidas, &columnas);
        let _ = self.mostrar_lineas_elegidas(lineas_elegidas, columnas);

        Ok("Proceso completo".to_string())
    }

    fn ordenar_lineas_elegidas(
        &self,
        lineas: &mut Vec<Vec<String>>,
        col: &Vec<String>,
    ) -> Result<String, MyError> {
        let index = match col.iter().position(|c| *c == self.order) {
            Some(i) => i,
            None => {
                return Err(MyError::InvalidColumn(
                    "Columna especificada para ordenar no existe en la tabla".to_string(),
                ))
            }
        };

        match self.asc {
            true => lineas.sort_by_key(|l| String::from(&l[index])),
            false => {
                lineas.sort_by_key(|l| String::from(&l[index]));
                lineas.reverse();
            }
        };

        Ok("Todo ok".to_string())
    }

    fn mostrar_lineas_elegidas(&self, lineas: Vec<Vec<String>>, col: Vec<String>) {
        match self.columnas.contains(&"*".to_string()) {
            true => println!("{}", col.join(", ")),
            false => println!("{}", self.columnas.join(", ")),
        }

        for l in &lineas {
            let mut aux = String::new();

            if self.columnas.contains(&String::from("*")) {
                aux = l.join(", ");
            } else {
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
            }
            println!("{}", aux);
        }
    }

    fn corroborar_columnas(&self, columnas: &Vec<String>) -> Result<String, MyError> {
        if self.columnas.len() == 1 && self.columnas.contains(&"*".to_string()) {
            return Ok("Todo correcto".to_string());
        }

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
            "".to_string(),
            false,
        );

        let select_esperado = Select {
            archivo: "./test/select.rs".to_string(),
            columnas: Vec::<String>::new(),
            condicion: Condicion::SiempreTrue,
            order: "".to_string(),
            asc: false,
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
        let select = Select::new(
            "./test/select.csv".to_string(),
            columnas,
            condicion,
            "".to_string(),
            false,
        );

        let resultado = select.seleccionar();

        assert!(resultado.is_ok());
    }

    #[test]
    pub fn test03_se_realiza_un_select_y_se_hace_un_orderby() {
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
        let select = Select::new(
            "./test/select.csv".to_string(),
            columnas,
            condicion,
            "cantidad".to_string(),
            false,
        );

        let resultado = select.seleccionar();

        assert!(resultado.is_ok());
    }
}
