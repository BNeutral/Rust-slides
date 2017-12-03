fn main() {
    fn max<'a, 'b: 'a>(x: &'a i32, y: &'b i32) -> &'a i32 {
        if *x > *y {    
            x
        } else {
            y
        }
    }
}