use std::cell::RefCell;
fn main() {
	let a = RefCell::new(String::from("Rust"));
	a.borrow_mut().push('!');
	println!("{}",a.borrow());
}

