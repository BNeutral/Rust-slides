fn seVuelveOwner(_s : String) {}

fn main() {
	let s = String::from("Rust");
	seVuelveOwner(s);
	println!("{}",s);	
}
