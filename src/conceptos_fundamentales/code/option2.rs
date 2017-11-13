#fn main() {
	let hayString = Some("Hay string!"); // Se infiere el tipo
	let noHayString: Option<String> = None; // Hay que especificar tipo
	match hayString {
	    Some(i) => println!("{}",i),
	    _ => (),
	}
	match noHayString {
	    Some(i) => println!("{}",i),
	    _ => (),
	}
#}
