fn main() {
    let num = {
        let mut sum = 0;
        for num in 0..16 { sum += num; }
        sum
    };
    println!("{:?}", num);
}