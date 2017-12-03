fn devuelveOwner(s : String) -> String {
	s
}

fn main() {
	let mut s = String::from("Rust");
	s = devuelveOwner(s);
	println!("{}",s);	
}
