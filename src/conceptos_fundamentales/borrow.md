# Borrow

Operador de referencia: &

* &Tipo : El tipo es una referencia inmutable

* &mut Tipo : El tipo es una referencia mutable

* &variable : Obtiene una referencia a la variable

* &mut variable : Obtiene una referencia mutable a la variable

* *variable : Desreferencia, si hace falta.

Se "pide prestado" el valor. Nos volvemos dueños, pero al final lo devolvemos a su dueño original.

Se mantienen las reglas de ownership.

✔️

{{#playpen ./code/borrow1.rs}}

✔️

{{#playpen ./code/borrow2.rs}}

## Pero y los métodos?

Los metodos toman &self como primer parametro y Rust hace desreferenciacion automática apropiada para que sea más lindo, en vez de tener varias sintaxis.

Se puede ver la equivalencia a continuación.

{{#playpen ./code/borrowmetref.rs}}

En otros casos, si hace falta desreferenciar explicitamente.

✔️

{{#playpen ./code/borrow4.rs}}

❌

{{#playpen ./code/borrow3.rs}}

## Reglas de borrows

* En un momento dado se puede tener de forma mutuamente exclusiva:

  * Una referencia mutable

  * N referencias inmutables

* Las referencias deben ser siempre validas

✔️

{{#playpen ./code/borrow5.rs}}

Sigue valiendo que solo se puede modificar el valor mutable si soy dueño.

{{#playpen ./code/borrow6.rs}}

❌

{{#playpen ./code/borrow7.rs}}
