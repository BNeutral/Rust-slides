#use std::rc::Rc;
#use std::cell::RefCell;
fn main() {
	let a = Rc::new(RefCell::new(String::from("Rust")));
	let b = Rc::clone(&a);
	let _ra = a.borrow_mut();
	let _rb = b.borrow_mut();
}

