use std::cell::RefCell;
fn main() {
	let a = RefCell::new(String::from("Rust"));
	let _b = a.borrow_mut();
	let _c = a.borrow();
}

