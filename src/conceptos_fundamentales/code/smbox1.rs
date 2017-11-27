fn main() {
	let mut a = Box::new(1);
	*a += 1;
	println!("a = {}", *a);
}
