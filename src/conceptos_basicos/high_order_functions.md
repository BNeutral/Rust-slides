# High Order Functions

```rust
fn mul(x: f64, n: f64) -> f64 { x * n }
fn sum(x: f64, n: f64) -> f64 { x + n }
fn div(x: f64, n: f64) -> f64 { x / n }
fn sub(x: f64, n: f64) -> f64 { x - n }

fn expr(base: f64, n: f64, functions: Vec<fn(f64, f64) -> f64>) -> f64 {
    let mut result = base;
    for func in functions.iter() {
        result = func(result, n);
    }
    result
}

let mut functions: Vec<fn(f64, f64) -> f64> = Vec::new();
functions.extend([sum, mul, sub, div].iter());
println!("{}", expr(3.0, 2.0, functions));  // (((3+2)*2)-2)/2 = 4
```
