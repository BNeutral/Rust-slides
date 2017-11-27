fn returns_five(x: i32) -> i32 {
    println!("returns_five invoked with argument: {}", x);
    5
}

fn returns_five_as_param(function: fn(i32) -> i32, argument: i32) {
    println!("returns_five_as_param invoked. Result {}", function(argument));
}

fn main() {
    returns_five_as_param(returns_five, 1);
}