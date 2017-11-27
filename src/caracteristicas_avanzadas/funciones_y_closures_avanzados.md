# Advanced Functions & Closures

## Function pointers

- Las funciones pueden recibir otras funciones como parámetro.

- Se debe indicar con el tipo `fn`, que tipo de argumento(s) recibe y que tipo devuelve (si lo hiciera) de la misma forma que se definen las funciones normalmente.

### Ejemplo

{{#playpen ./code/function_pointers.rs}}

---

## Returning Closures

- __Rust__ no permite utilizar __closures__ cómo return type de una función porque no puede saber cuanto espacio va a necesitar para almacenar dicho closure.

- Se debe encapsular haciendo uso de un smart pointer.

### Ejemplo

{{#playpen ./code/returning_closures.rs}}
