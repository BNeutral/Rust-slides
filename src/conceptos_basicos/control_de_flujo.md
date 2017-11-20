# Control de flujo

* `if / else` es una expresión
* `loop` con `break` y `continue`
* `while`
* `for x in 0..10` para usar con iteradores
* `match`
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
