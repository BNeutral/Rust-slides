#use std::io;
#use std::fs::File;
fn main() { 
    println!("{:?}",leerArchivo()); 
}

fn leerArchivo() -> Result<String, io::Error> {
    let f = File::open("noExiste.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    // Imaginar que sacamos info del archivo
    Ok(String::new())
}
