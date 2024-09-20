use sql_rustico::parser::Parser;
use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let direccion = args.remove(1);
    let instruccion = args.remove(1);

    let mut parser = Parser::new();

    let operacion = match parser.crear_operacion(direccion, instruccion) {
        Ok(o) => o,
        Err(e) => return println!("{}", e),
    };

    match operacion.realizar_operacion() {
        Ok(m) => println!("{}", m),
        Err(e) => println!("{}", e),
    };
}
