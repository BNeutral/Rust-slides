# Métodos

* No clases de cajon, sí los mecanismos de POO

```rust
struct Point { x: f64, y: f64 }

impl Point {
    fn new(x: f64, y: f64) -> Point { //associated function = static
        Point { x: x, y: y }
    }

    fn distance_to_origin(&self) -> f64 {
        f64::sqrt(f64::powf(self.x, 2.0) + f64::powf(self.y, 2.0))
    }
}

fn main() {
    let c = Point::new(4.0, -3.0);
    println!("{}", c.distance_to_origin());
}
```
