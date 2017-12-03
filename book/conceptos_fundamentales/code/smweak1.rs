#use std::rc::Rc;
fn main() {
	let a = Rc::new(String::from("Rust"));
	let b = Rc::clone(&a);
	let c = Rc::downgrade(&a);
	println!("strong = {}, weak = {}",Rc::strong_count(&a),Rc::weak_count(&a));
	println!("{:?}",b);
	println!("{:?}",c);
	println!("{:?}",c.upgrade());
	drop(a);
	drop(b);
	println!("{:?}",c.upgrade());
}

