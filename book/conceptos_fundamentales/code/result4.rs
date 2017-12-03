#use std::fs::File;
#fn main() {
    File::open("noExiste.txt").expect("Un mensaje");
#}
