#use std::rc::Rc;
#use std::cell::RefCell;
fn main() {
	let a = Rc::new(RefCell::new(String::from("Rust")));
	let b = Rc::clone(&a);
	let c = Rc::clone(&a);
	a.borrow_mut().push('!');
	b.borrow_mut().push('!');
	c.borrow_mut().push('!');
	println!("{}",a.borrow());
}

