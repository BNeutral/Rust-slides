use std::rc::Rc;
fn main() {
	let a = Rc::new(String::from("Rust"));
	let b = Rc::clone(&a);
	let c = Rc::clone(&a);
	println!("{}-{}-{}",a,b,c);
	println!("#refs: {}",Rc::strong_count(&a));
}
