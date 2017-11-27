#use std::fs::File;
#use std::io::ErrorKind;
#fn main() {
    let f = File::open("noExiste.txt");
    match f {
        Ok(_file) => println!("OK"),
        Err(ref _error) if _error.kind() == ErrorKind::NotFound 
		=> { println!("{:?}",_error) },
        Err(_error) => { panic!() },
    };
#}
