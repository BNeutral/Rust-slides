# Binding

* `let immutable_binding = 1`
* `let mut mutable_binding = 1`
* Scope & Shadowing
* Se puede solo declarar, para luego inicializar
    * Compilador prohíbe uso de variables no inicializadas
* Inferencia de tipos en base de uso
* Alias:
    ```
    type Dollar = f64;
    let cookie_price: Dollar = 1.43;
    ```
* `const N: i32 = 5`
* `static N: i32 = 5`