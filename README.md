# sql_rustico

## Instrucciones permitidas:
Aquello que aparezca entre \[\] puede o no estar en la instrucción.
- **INSERT**
    *Ejemplo*
    cargo run -- url "INSERT INTO tabla (col1, col2, col3, ...) VALUES (val1, val2, val3, ...), ..." (*Agregar más de un dato es opcional*)
- **DELETE**
    *Ejemplo*
    cargo run -- url "DELETE FROM tabla \[WHERE condición\]"
- **UPDATE**
    *Ejemplo*
    cargo run -- url "UPDATE FROM tabla SET col1=val1, col2=val2, ... WHERE condición"
- **SELECT**
    *Ejemplo*
    cargo run -- url "SELECT col1, col2, ... FROM tabla \[WHERE condición\] \[ORDER BY columna \[desc\]\]"

## Aclaraciones:
- Si el valor que se desea insertar o poner en alguna condición es un string no se tiene que poner entre ''.
