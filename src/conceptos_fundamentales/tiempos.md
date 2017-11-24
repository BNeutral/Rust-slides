# Lifetimes

Concepto de Rust: Poder saber si seguira siendo valido lo que estamos refiriendo.

{{#playpen ./code/lifetest.rs}}

❌

{{#playpen ./code/life0.rs}}

❌

{{#playpen ./code/life1.rs}}

❌

{{#playpen ./code/life2.rs}}

El lifetime es necesario para saber cuando terminar el borrow.

## Sintaxis de lifetime

* fn funcion<'a> especifica un tiempo de vida genérico para asociar 

* &'a Tipo: Referencia con tiempo de vida explicito

Nota: Si dos parametros de entrada tienen el mismo lifetime asignado, el valor tomado será el más chico

✔️

{{#playpen ./code/life4.rs}}

✔️

{{#playpen ./code/life3.rs}}

Los casos típicos son inferidos por el compilador.

✔️

{{#playpen ./code/life5.rs}}

El concepto también aplica a structs con referencias

{{#playpen ./code/life6.rs}}
