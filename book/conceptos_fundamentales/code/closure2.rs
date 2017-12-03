#fn main() {
	let a = String::from("A");
	let f = || { println!("{}",a); }; // Borrow inmutable
	let _c = a; // Intento tomar ownership
	f();
#}
