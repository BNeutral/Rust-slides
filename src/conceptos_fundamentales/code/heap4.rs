struct Wrapper {
    s: String,
}

impl Wrapper {
    fn append(&self, c : char) {
        self.s.push(c);
    }
}

fn main() {
	let mut a = Wrapper { s : String::from("Hola") };
    	let b = a;
	b.append('!');
	println!("a:{} b:{}",a,b);
}
