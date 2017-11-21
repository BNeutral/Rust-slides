#fn main() {
	let mut v1 = 1;
	let v2 = &mut v1;
	*v2 += 1;
	println!("v1:{}",v1);
#}
