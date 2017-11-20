# Traits (1)

```rust
struct Square { side: f64 }
struct Circle { radius: f64 }
trait Shape {
    fn area(&self) -> f64;
}

impl Shape for Square {
    fn area(&self) -> f64 { self.side * self.side }
}

impl Shape for Circle {
    fn area(&self) -> f64 { std::f64::consts::PI * (self.radius * self.radius) }
}

let s = Square { side: 2.5 };
let c = Circle { radius: 5.0 };
println!("s.area: {s_a}, c.area: {c_a}", s_a = s.area(), c_a = c.area());
```