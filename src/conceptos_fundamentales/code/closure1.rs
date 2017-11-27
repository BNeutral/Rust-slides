#fn main() {
	let a = String::from("A");
	let f = || { println!("{}",a); };
	let g = |p1:i32| p1+1;
	let h = |p1,p2| -> i32 { p1+p2 };
	f();
	println!("{}",g(1));
	println!("{}",h(1,1));
#}
