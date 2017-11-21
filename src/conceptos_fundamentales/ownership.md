# Ownership y move

## Las reglas de Rust

1. Todo valor en Rust tiene una variable que se llama owner.
2. En un dado momento, solo puede haber un owner.
3. Cuando el owner sale de scope, se hace drop de la variable.

Estas reglas son obligatorias y chequeadas por el compilador.

## Move

Se llama move al cambio de owner. Pasar el valor, retornarlo, o bindearlo a otra variable cambia el owner.

❌

{{#playpen ./code/own1.rs}}

❌

{{#playpen ./code/own2.rs}}

❌

{{#playpen ./code/own3.rs}}

✔️

{{#playpen ./code/own4.rs}}
