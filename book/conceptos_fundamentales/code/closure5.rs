#fn main() {
	let mut a = String::from("Rust");
	{
		let mut f = move || { 
			a.push('!');
			println!("{}",a);
		};
		f();
	}
	println!("{}",a);
#}
