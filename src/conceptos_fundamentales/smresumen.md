# Resumen

* Box: Tamaño fijo, contiene algo que esta en el heap.

* Rc: Permite multiples owners inmutables. Single threaded.

* Weak: Variante del Rc para no leakear memoria en ciclos.

* RefCell: Permite romper las reglas de borrows en tiempo de compilación, aunque no de ejecución. Single threaded.

## Otros que no vimos

* Cell: Lo mismo que el refcell pero para valores copy. No causa panics porque no hace falta sin referencias.

* Arc: Version thread safe de Rc. Muchas veces el tipo que llevan dentro es un atomic.
