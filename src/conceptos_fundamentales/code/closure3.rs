fn llama<F>(mut f : F) where
	F: FnMut(char) {
		f('!');
	}

fn main() {
	let mut a = String::from("Rust");
	let f = |p1| { 
		a.push(p1);
		println!("{}",a);
	};
	llama(f);
}
