use crate::delete::Delete;
use crate::insert::Insert;
use crate::my_error::MyError;
use crate::select::Select;
use crate::update::Update;

#[derive(Debug)]
pub enum Operacion {
    Insert(Insert),
    Delete(Delete),
    Update(Update),
    Select(Select),
}

impl Operacion {
    pub fn realizar_operacion(&self) -> Result<String, MyError> {
        return match self {
            Operacion::Insert(insert) => insert.insertar(),
            Operacion::Delete(delete) => delete.eliminar(),
            Operacion::Update(update) => update.update(),
            Operacion::Select(select) => select.seleccionar(),
        };
    }
}
