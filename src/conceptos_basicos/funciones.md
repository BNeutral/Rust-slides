# Funciones

```
fn foo(x: i32) -> i32 {
    ...
    //expression
}
```
* Obligatorio indicar los tipos
* Existe `return`, se considera poco sano
* Funciones divergentes: `fn diverges() -> ! {...}`
    * Nunca se devuelven, distinto de devolver "nada" (unit type, `()`)
    * Para wrappear loops infinitos o codigo que termina el programa
* Binding de función a una variable
    ```rust
    fn plus_one(i: i32) -> i32 { i + 1 }
    let f = plus_one;
    println!("{}", f(5));
    ```