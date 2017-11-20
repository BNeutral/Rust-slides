# Lazy

* No existe en forma nativa (hay librería hecha por comunidad)
* Internamente usa lazy para iteradores
    ```rust
    // suma de cuadrados de numeros impares hasta un limite
    let limit = 10;

    let result: u32 = (0..)
        .take_while(|&n| n <= limit)
        .filter(|&n| n % 2 == 1)
        .map(|n| n * n)
        .fold(0, |sum, i| sum + i);

    println!("{}", result);
    ```


