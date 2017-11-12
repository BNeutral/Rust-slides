# Expresiones

Rust es principalmente un lenguaje de expresiones.

Solo hay 2 tipos de statements:
* Declaration statement (`let`)
* Expression statement (evalua expresión e ignora su resultado)
    * Expresion cuyo resultado se reprime con `;` se evalúa a `()`
    * *Block expression* o *control flow expression* que no termina con expresión

{{#playpen ./expresiones.rs}}