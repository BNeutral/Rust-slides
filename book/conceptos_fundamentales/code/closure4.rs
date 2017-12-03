#fn main() {
	let mut a = String::from("Rust");
	{
		let mut f = || { 
			a.push('!');
			println!("{}",a);
		};
		f();
	}
	println!("{}",a);
#}
