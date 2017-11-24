fn fun(a:&String, b:&String) -> &String {
	&a	
}

fn main() {
	let a = String::new();
	let b = String::new();
	let c = fun(a,b);
}
