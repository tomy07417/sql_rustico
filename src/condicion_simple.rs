use crate::my_error::MyError;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Valor {
    Entero(i32),
    Palabra(String),
}

#[derive(Debug, PartialEq)]
pub struct CondicionSimple {
    columna: String,
    simbolo: String,
    valor: Valor,
    es_int: bool
}


impl CondicionSimple {
    pub fn new(columna:String, simbolo:String, valor:String) -> Self {
        return match valor.parse::<i32>() {
            Ok(v) => CondicionSimple{columna, simbolo, valor:Valor::Entero(v), es_int:true},
            Err(_e) => CondicionSimple{columna, simbolo, valor:Valor::Palabra(valor), es_int:false},
        };
    }

    pub fn verificar(&self, cols:&Vec<String>, valores:&Vec<String>) -> Result<bool, MyError> {
        let index = match cols.iter().position(|p| *p == self.columna) {
            Some(p) => p,
            None => return Err(MyError::InvalidColumn("La columna seleccionada para la condición no existe en la tabla".to_string())),
        }; 
        
        if ! self.es_int == valores[index].parse::<i32>().is_ok() {
            return Err(MyError::InvalidColumn("El tipo de dato que le corresponde a la columna especificada en la condición no es el utilizado".to_string()))
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
            _ => return Err(MyError::InvalidSyntax("El simbolo utilizado en la operación condicional no existe".to_string())),
        };

        condicion
    }

}


#[test]
pub fn test01_se_crea_una_condicion_simple_correctamente() {
    let condicion = CondicionSimple::new("nombre".to_string(), "=".to_string(), "Tomas".to_string());

    let condicion_esperada = CondicionSimple {columna: "nombre".to_string(), simbolo:"=".to_string(), valor:Valor::Palabra("Tomas".to_string()), es_int: false};

    assert_eq!(condicion, condicion_esperada);
}

#[test]
pub fn test02_se_verifica_si_la_condicion_se_cumple_utilizando_igual_y_se_cumple() {
    let condicion = CondicionSimple::new("nombre".to_string(), "=".to_string(), "Tomas".to_string());

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

