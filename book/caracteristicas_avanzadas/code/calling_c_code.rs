#[link(name = "c_file")]
extern {
    fn c_function();
}

fn safe_call_to_c_function() {
    println!("Calling c function");
    unsafe {
        c_function();
    }
}

fn main() {
    safe_call_to_c_function();
}