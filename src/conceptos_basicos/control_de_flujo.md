# Control de flujo

* `if / else` es una expresión
* `loop` con `break` y `continue`
* `while`
* `for x in 0..10` para usar con iteradores
* `if let` para evaluar desestructuración y asignar al mismo tiempo
    ```rust
    let hayString = Some("Hay string!");
    if let Some(text) = hayString {
        println!("{}", text);
    }
    ```
* `while let`
    ```
    let mut x = BoardGameAction::Move { squares: 1 };
    while let BoardGameAction::Move { squares } = x { //x changes }
    ```
