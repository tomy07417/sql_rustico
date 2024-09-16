use crate::condicion_simple::CondicionSimple;
use crate::my_error::MyError;

pub enum Condicion {
    CondicionSimple(CondicionSimple),
    And(Box<Condicion>, Box<Condicion>),
    Or(Box<Condicion>, Box<Condicion>),
    Not(Box<Condicion>),
}

impl Condicion {
    pub fn verificar(&self, columnas:&Vec<String>, valores:&Vec<String>) -> Result<bool, MyError> {
        match self {
            Condicion::CondicionSimple(cond) => return cond.verificar(columnas, valores),
            Condicion::And(cond1, cond2) => {
                let c1 = match cond1.verificar(columnas, valores) {
                    Ok(c) => c,
                    Err(e) => return Err(e),
                };

                let c2 = match cond2.verificar(columnas, valores) {
                    Ok(c) => c,
                    Err(e) => return Err(e),
                };

                return Ok(c1 && c2)
            },

            Condicion::Or(cond1, cond2) => {
                let c1 = match cond1.verificar(columnas, valores) {
                Ok(c) => c,
                Err(e) => return Err(e),
                };

                let c2 = match cond2.verificar(columnas, valores) {
                Ok(c) => c,
                Err(e) => return Err(e),
                };

                return Ok(c1 || c2)
            },
            Condicion::Not(cond) => {
                let c = match cond.verificar(columnas, valores) {
                Ok(c) => c,
                Err(e) => return Err(e),
                };

                return Ok(!c)
            },
        }
    }
}


#[test]
pub fn test01_se_verifica_si_una_condicon_simple_devuelve_el_verdadero() {
        let condicion_simple = CondicionSimple::new("nombre".to_string(), "=".to_string(), "Tomas".to_string());
        let condicion = Condicion::CondicionSimple(condicion_simple);

        let columnas = vec!["nombre".to_string(), "apellido".to_string()];
        let valores = vec!["Tomas".to_string(), "Amundarain".to_string()];
        
        let resultado = match condicion.verificar(&columnas, &valores) {
            Ok(r) => r,
            Err(_e) => false,
        };

        assert!(resultado);
}

#[test]
pub fn test02_se_verifica_si_una_condicon_and_devuelve_el_verdadero() {
        let condicion_simple1 = Condicion::CondicionSimple(CondicionSimple::new("nombre".to_string(), "=".to_string(), "Tomas".to_string()));
        let condicion_simple2 = Condicion::CondicionSimple(CondicionSimple::new("apellido".to_string(), "=".to_string(), "Amundarain".to_string()));
        let condicion = Condicion::And(Box::new(condicion_simple1), Box::new(condicion_simple2));

        let columnas = vec!["nombre".to_string(), "apellido".to_string()];
        let valores = vec!["Tomas".to_string(), "Amundarain".to_string()];
        
        let resultado = match condicion.verificar(&columnas, &valores) {
            Ok(r) => r,
            Err(_e) => false,
        };

        assert!(resultado);
}

#[test]
pub fn test03_se_verifica_si_una_condicon_or_devuelve_el_verdadero() {
        let condicion_simple1 = Condicion::CondicionSimple(CondicionSimple::new("nombre".to_string(), "=".to_string(), "Tomas".to_string()));
        let condicion_simple2 = Condicion::CondicionSimple(CondicionSimple::new("apellido".to_string(), "=".to_string(), "Amundarain".to_string()));
        let condicion = Condicion::Or(Box::new(condicion_simple1), Box::new(condicion_simple2));

        let columnas = vec!["nombre".to_string(), "apellido".to_string()];
        let valores1 = vec!["Francisco".to_string(), "Amundarain".to_string()];
        let valores2 = vec!["Tomas".to_string(), "Martinez".to_string()];

        let resultado1 = match condicion.verificar(&columnas, &valores1) {
            Ok(r) => r,
            Err(_e) => false,
        };

        let resultado2 = match condicion.verificar(&columnas, &valores2) {
            Ok(r) => r,
            Err(_e) => false,
        };

        assert!(resultado1);
        assert!(resultado2);
}

#[test]
pub fn test04_se_verifica_si_una_condicon_not_devuelve_el_verdadero() {
        let condicion_simple1 = Condicion::CondicionSimple(CondicionSimple::new("nombre".to_string(), "=".to_string(), "Tomas".to_string()));
        let condicion_simple2 = Condicion::CondicionSimple(CondicionSimple::new("apellido".to_string(), "=".to_string(), "Amundarain".to_string()));
        let condicion = Condicion::Not(Box::new(condicion_simple2));

        let columnas = vec!["nombre".to_string(), "apellido".to_string()];
        let valores = vec!["Tomas".to_string(), "Martinez".to_string()];

        let resultado = match condicion.verificar(&columnas, &valores) {
            Ok(r) => r,
            Err(_e) => false,
        };

        assert!(resultado);
}

