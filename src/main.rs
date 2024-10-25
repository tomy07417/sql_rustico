use sql_rustico::my_error::MyError;
use sql_rustico::parser::Parser;
use std::env;

fn main() -> Result<(), MyError> {
    let mut args: Vec<String> = env::args().collect();
    let direccion = args.remove(1);
    let instruccion = args.remove(1);

    let mut parser = Parser::new();

    match parser.crear_operacion(direccion, instruccion) {
        Ok(o) => {
            let operacion = o.realizar_operacion();
            match operacion {
                Ok(_) => {}
                Err(e) => println!("{}", e),
            };
        }
        Err(e) => println!("{}", e),
    };

    Ok(())
}
