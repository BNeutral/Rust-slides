static a : &str = "Rust";

fn referencia() -> &str {
	&a	
}

fn main() {
	let b = referencia();
}
