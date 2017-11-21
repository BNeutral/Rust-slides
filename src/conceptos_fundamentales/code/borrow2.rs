fn pidePrestado(s : &mut String) { // Mutable
	s.push('!');
}

fn main() {
	let mut s = String::from("Rust");
	pidePrestado(&mut s);
	println!("{}",s);	
}
