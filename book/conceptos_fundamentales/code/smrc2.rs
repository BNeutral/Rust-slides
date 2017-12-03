use std::rc::Rc;
fn main() {
	let mut a = Rc::new(String::from("Rust"));
	a.push('!');
}
