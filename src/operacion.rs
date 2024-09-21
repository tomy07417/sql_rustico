use crate::delete::Delete;
use crate::insert::Insert;
use crate::my_error::MyError;
use crate::select::Select;
use crate::update::Update;

///# Operacion
///Esta estructura es la que proporciona toda la funcionalida para realizar las diferentes
///instrucciones que se soportan de sql.
///
///**Ejemplo**
///let operacion = Operacion::Insert(Delete::new(direccion_archivo, condicion));
///operacion.realizar_operacion();
///
///**Tipos**
///- 'Insert': Es la operación que representa a la instrucción INSERT.
///- 'Delete': Es la operación que representa a la instrucción DELETE.
///- 'Update': Es la operación que representa a la instrucción UPDATE.
///- 'Select': Es la opereción que representa a la instrucción SELECT.
#[derive(Debug)]
pub enum Operacion {
    Insert(Insert),
    Delete(Delete),
    Update(Update),
    Select(Select),
}

impl Operacion {
    ///# Operacion.realizar_operacion()
    ///Esta función realiza la instrucción que se desea al ejecutar el programa.
    ///
    ///**Return**
    ///Devuelve un *Result<String, MyError>* en caso que durante la ejecución de la función no haya
    ///ocurrido ningún error devuelve un *String* para indicar que la operación se realizó
    ///correctamente, en caso contrario se devuelve un error de tipo *MyErroMyErrorr*.
    pub fn realizar_operacion(&self) -> Result<String, MyError> {
        return match self {
            Operacion::Insert(insert) => insert.insertar(),
            Operacion::Delete(delete) => delete.eliminar(),
            Operacion::Update(update) => update.update(),
            Operacion::Select(select) => select.seleccionar(),
        };
    }
}
