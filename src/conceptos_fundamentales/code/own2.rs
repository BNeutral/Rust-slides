#fn main() {
	let s1 = String::new();
	{
		let _s2 = s1;
	}
	println!("{}",s1);
#}
