fn fun<'a>(a : &'a String, b : &String) -> &'a String {
	&a	
}

fn main() {
	let a = String::new();
	let b = String::new();
	let c = fun(&a,&b);
}
