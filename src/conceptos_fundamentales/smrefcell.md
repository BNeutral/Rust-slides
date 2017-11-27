# RefCell

Similar a un Box, tambien solo para single threaded, pero permite compilar rompiendo las reglas de borrows.

Las reglas aun deben cumplirse en tiempo de ejecución, de no hacerlo hay un panic!.

## Pattern de mutabilidad interior

Mediante borrow() y borrow_mut() podemos elegir que tipo de borrow queremos hacer en tiempo de ejecución, pese a que la refcell sea inmutable.

❌
{{#playpen ./code/smrefcell1.rs}}

✔️
{{#playpen ./code/smrefcell2.rs}}

✔️
{{#playpen ./code/smrefcell3.rs}}

❌
{{#playpen ./code/smrefcell4.rs}}

## Combinado con Rc

Nos permite tener multiples borrows mutables 😲

Hay que seguir respetando las reglas de runtime.

✔️
{{#playpen ./code/smrefcell5.rs}}

❌
{{#playpen ./code/smrefcell6.rs}}
