#use std::fs::File;
#use std::io::ErrorKind;
#fn main() {
    let f = File::open("noExiste.txt");
    match f {
        Ok(file) => println!("OK"),
        Err(ref error) if error.kind() == ErrorKind::NotFound 
		=> { println!("{:?}",error) },
        Err(error) => { panic!() },
    };
#}
