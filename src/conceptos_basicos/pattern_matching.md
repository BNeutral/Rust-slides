# Pattern matching

* Compilador exige exhaustividad: `_`
* También es expresión
    ```
    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1
    };
    ```
* Soporta desestructuración
* Numerosas variedades:
    * `'r'`
    * `3 | 4 | 5`
    * `num @ 5...29`
    * `(0, y)`
    * `Color::RGB(r, g, b)`
    * `Point { x, y, _ }`
    * *match guards* para filtrar ramas: `(x, y) if x + y == 0`