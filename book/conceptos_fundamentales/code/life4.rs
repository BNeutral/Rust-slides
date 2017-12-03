static a : &str = "Rust";

fn referencia() -> &'static str {
	&a	
}

fn main() {
	let b = referencia();
}
