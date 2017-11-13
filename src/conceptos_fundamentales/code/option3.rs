#fn main() {
	let hayString = Some("Hay string!");
	let noHayString: Option<String> = None;
	if let Some(i) = hayString { println!("{}",i); }
	if let Some(i) = noHayString { println!("{}",i); }
#}
