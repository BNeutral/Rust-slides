# Control de flujo

* `if / else` es una expresión
* `loop` con `break` y `continue`
* `while`
* `for` para usarlo con iteradores
* `match` para pattern matching
* `if let` para evaluar desestructuración y asignar al mismo tiempo
    ```
    if let BoardGameAction::Move { squares } = x {
        println!("{:?}", squares);
    } else { ... }
    ```
* `while let`
    ```
    let mut x = BoardGameAction::Move { squares: 1 };
    ...
    while let BoardGameAction::Move { squares } = x {
        //x cambia en el cuerpo de while
    }
    ```
