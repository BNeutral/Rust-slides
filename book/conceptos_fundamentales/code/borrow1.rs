fn pidePrestado(_s : &String) { } // Inmutable

fn main() {
	let s = String::from("Rust");
	pidePrestado(&s);
	println!("{}",s);	
}
