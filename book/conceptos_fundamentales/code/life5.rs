fn fun(a : & String) -> & String {
	&a	
}

fn main() {
	let a = String::new();
	let c = fun(&a);
}
