use crate::my_error::MyError;
use crate::valor::Valor;

///# CondicionSimple
///Esta es la estructura que proporciona soporte para las operaciones lógica básicas
///
///**Ejemplo**
///let cond = CondicionSimple::new(nombre_columna, =, 4);
///let resultado:bool = cond.verificar(columnas, valores);
///
///**Parámetros**
///- 'columna': Es un **String** que representa al nombre de la columna al que representa al dato
///que se quiere comparar.
///- 'simbolo': Es un **String** que representa al tipo de opreción lógica que se espera realizar
///en cada comparación para ver si se cumple o no la condición.
///- 'valor': Es el valor contra el que se compara cada dato que se quiere ver si cumple la
///condicion.
///- 'es_int': Es un **bool** que sirve para corroborar si el tipo de dato que se quiere comparar
///debe ser un *Int* o *String*.
#[derive(Debug, PartialEq)]
pub struct CondicionSimple {
    columna: String,
    simbolo: String,
    valor: Valor,
    es_int: bool,
}

impl CondicionSimple {
    ///# CondicionSimple.new()
    ///Esta función crea una nueva instacia de CondicionSimple.
    ///
    ///**Parámetros**
    ///- 'columna': Es la columna a la que va a representar el valor que se quiere comparar.
    ///- 'simbolo': Es el tipo de opreción lógica que se quiere realizar.
    ///- 'valor': Es el el valor contra el que se van a comparar las distintas filas para ver si
    ///cumplen o no la condición.
    ///
    ///**Return**
    ///Devuelve un *Struct* de tipo *CondicionSimple*
    pub fn new(columna: String, simbolo: String, valor: String) -> Self {
        return match valor.parse::<i32>() {
            Ok(v) => CondicionSimple {
                columna,
                simbolo,
                valor: Valor::Entero(v),
                es_int: true,
            },
            Err(_e) => CondicionSimple {
                columna,
                simbolo,
                valor: Valor::Palabra(valor),
                es_int: false,
            },
        };
    }
    ///# CondicionSimple.verificar()
    ///Esta función verifica si la fila que se le pasa cumple la condición o no
    ///
    ///**Parámetros**
    ///- 'cols': Es un array que representa a los nombres de las columnas de la tabla a la que
    ///representa *'valores'*.
    ///- 'valores': Es un array que contiene los valores por columna de una fila de una tabla
    ///determinada.
    ///
    ///**Return**
    ///Devuelve un Result<bool, MyError>, en caso de que no haya ocurrido ningún error en la
    ///ejecución de la función se devuelve el *bool*, en caso contrario se devuelve un error de
    ///tipo *MyError*.
    pub fn verificar(&self, cols: &Vec<String>, valores: &Vec<String>) -> Result<bool, MyError> {
        let index = match cols.iter().position(|p| *p == self.columna) {
            Some(p) => p,
            None => {
                return Err(MyError::InvalidColumn(
                    "La columna seleccionada para la condición no existe en la tabla".to_string(),
                ))
            }
        };

        if !self.es_int == valores[index].parse::<i32>().is_ok() {
            return Err(MyError::InvalidColumn("El tipo de dato que le corresponde a la columna especificada en la condición no es el utilizado".to_string()));
        }

        let aux = match self.es_int {
            true => Valor::Entero(match valores[index].parse::<i32>() {
                Ok(v) => v,
                Err(_e) => return Err(MyError::InvalidColumn("El tipo de dato que le corresponde a la columna especificada en la condición no es el utilizado".to_string())),
            }),
            false => Valor::Palabra(String::from(&valores[index])),
        };
        let condicion = match self.simbolo {
            _ if String::from("=") == self.simbolo => Ok(aux == self.valor),
            _ if String::from(">") == self.simbolo => Ok(aux > self.valor),
            _ if String::from("<") == self.simbolo => Ok(aux < self.valor),
            _ if String::from("!=") == self.simbolo => Ok(aux != self.valor),
            _ if String::from("<=") == self.simbolo => Ok(aux <= self.valor),
            _ if String::from(">=") == self.simbolo => Ok(aux >= self.valor),
            _ => {
                return Err(MyError::InvalidSyntax(
                    "El simbolo utilizado en la operación condicional no existe".to_string(),
                ))
            }
        };

        condicion
    }
}

#[test]
pub fn test01_se_crea_una_condicion_simple_correctamente() {
    let condicion =
        CondicionSimple::new("nombre".to_string(), "=".to_string(), "Tomas".to_string());

    let condicion_esperada = CondicionSimple {
        columna: "nombre".to_string(),
        simbolo: "=".to_string(),
        valor: Valor::Palabra("Tomas".to_string()),
        es_int: false,
    };

    assert_eq!(condicion, condicion_esperada);
}

#[test]
pub fn test02_se_verifica_si_la_condicion_se_cumple_utilizando_igual_y_se_cumple() {
    let condicion =
        CondicionSimple::new("nombre".to_string(), "=".to_string(), "Tomas".to_string());

    let cols = vec!["nombre".to_string(), "apellido".to_string()];
    let valores = vec!["Tomas".to_string(), "Amundarain".to_string()];

    let resultado = match condicion.verificar(&cols, &valores) {
        Ok(r) => r,
        Err(_r) => false,
    };

    assert!(resultado);
}

#[test]
pub fn test03_se_verifica_si_la_condicion_se_cumple_utilizando_mayor_y_se_cumple() {
    let condicion = CondicionSimple::new("valor".to_string(), ">".to_string(), "3".to_string());

    let cols = vec!["nombre".to_string(), "valor".to_string()];
    let valores = vec!["Tomas".to_string(), "4".to_string()];

    let resultado = match condicion.verificar(&cols, &valores) {
        Ok(r) => r,
        Err(_r) => false,
    };

    assert!(resultado);
}

#[test]
pub fn test04_se_verifica_si_la_condicion_se_cumple_utilizando_menor_y_se_cumple() {
    let condicion = CondicionSimple::new("valor".to_string(), "<".to_string(), "10".to_string());

    let cols = vec!["nombre".to_string(), "valor".to_string()];
    let valores = vec!["Tomas".to_string(), "4".to_string()];

    let resultado = match condicion.verificar(&cols, &valores) {
        Ok(r) => r,
        Err(_r) => false,
    };

    assert!(resultado);
}

#[test]
pub fn test05_se_verifica_si_la_condicion_se_cumple_utilizando_diferente_y_se_cumple() {
    let condicion = CondicionSimple::new("valor".to_string(), "!=".to_string(), "3".to_string());

    let cols = vec!["nombre".to_string(), "valor".to_string()];
    let valores = vec!["Tomas".to_string(), "4".to_string()];

    let resultado = match condicion.verificar(&cols, &valores) {
        Ok(r) => r,
        Err(_r) => false,
    };

    assert!(resultado);
}

#[test]
pub fn test06_se_verifica_si_la_condicion_se_cumple_utilizando_menoroigualque_y_se_cumple() {
    let condicion = CondicionSimple::new("valor".to_string(), "<=".to_string(), "4".to_string());

    let cols = vec!["nombre".to_string(), "valor".to_string()];
    let valores = vec!["Tomas".to_string(), "4".to_string()];

    let resultado = match condicion.verificar(&cols, &valores) {
        Ok(r) => r,
        Err(_r) => false,
    };

    assert!(resultado);
}

#[test]
pub fn test07_se_verifica_si_la_condicion_se_cumple_utilizando_mayoroigualque_y_se_cumple() {
    let condicion = CondicionSimple::new("valor".to_string(), ">=".to_string(), "4".to_string());

    let cols = vec!["nombre".to_string(), "valor".to_string()];
    let valores = vec!["Tomas".to_string(), "4".to_string()];

    let resultado = match condicion.verificar(&cols, &valores) {
        Ok(r) => r,
        Err(_r) => false,
    };

    assert!(resultado);
}
