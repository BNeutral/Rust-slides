#fn main() {
	let mut s = String::from("Rust");
	String::push(&mut s, '!');
	(&mut s).push('!');
	s.push('!');
	println!("{}",s)
#}
