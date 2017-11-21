# Tipos básicos (2)

* Tuples: `(2, 5.8, 's')`
* Structs: `struct Point { x: f64, y: f64 }`
    * Puede ser unit-like: `struct Comparator`
* Tuple structs: `struct Point(i32, i32, i32)`
* Enums:
    ```
    enum BoardGameAction {
        Move { squares: i32 },
        Surrender,
        Chat(String)
    }
    ```
* Rust no tiene null. Hay unit type, `()`, cuyo único valor es `()`. Similar a un void.
    ```rust
    let a: () = ();
    println!("{:?}", a)
    ```