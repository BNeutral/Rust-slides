# Option

Similar al Maybe de Haskell o Nullable de C#, tiene algo o no tiene nada
```
enum Option<T> {
    Some(T),
    None
}
```

```rust
fn checked_division(dividend: f64, divisor: f64) -> Option<f64> {
    if divisor == 0.0 { None }
    else { Some(dividend / divisor) }
}
// Se usa pattern matching para leer los resultados
match checked_division(5.0, 0.0) {
    Some(result) => println!("result: {}", result),
    None => println!("failed!")
}
```
