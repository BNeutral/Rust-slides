# Tipos básicos

* Strings (UTF-8): `&str` y `String`
* Arrays: `[1, 2, 3]`, `[0; 5]`
* Slices: `&mi_vector[1..4]`
* Tuples:
    * Comunes: `(2, 5.8, 's')`
    * Unit type (tupla vacía): `()`
* Structs: `struct Point { x: f64, y: f64 }`
    * Puede ser Unit struct: `struct Comparator`
* Tuple structs: `struct Point(i32, i32, i32)`
* Enums:
    ```
    enum BoardGameAction {
        Move { squares: i32 },
        Surrender,
        Chat(String)
    }
    ```
