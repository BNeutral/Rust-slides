#use std::fmt;
##[derive(Debug)]
enum ListaEstiloOZ<T>{
	Val(T, Box<ListaEstiloOZ<T>>),
	Nil, 
}
#use ListaEstiloOZ::Val;
#use ListaEstiloOZ::Nil;

fn imprimir<T : fmt::Display>(lista : ListaEstiloOZ<T>) {
	match lista {
		Nil => println!("Nil"),
		Val(a,b) => { println!("{}",a); imprimir(*b);},
	}		
}

fn main(){
	let a = Val(1, Box::new(Val(2, Box::new(Nil))));
	println!("{:?}",a);
	imprimir(a);
}

