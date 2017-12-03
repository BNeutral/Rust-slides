fn main() {
    let a;         // -------+- 'a
    {              //        |
        let b = 5; // -+- 'b |
        a = &b;    //  |     |
    }              // -+     |
    println!("a: {}", a);// -+
}
