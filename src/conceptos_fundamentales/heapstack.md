# Heap y Stack

## Stack

Se alocan los tipos básicos numéricos de tamaño conocido en tiempo de compilación.

* Enteros
* Floats
* Bools
* Tuplas que contengan solo estos tipos

Luego se los pasa por copia.

{{#playpen ./code/heap1.rs}}

## Heap

El resto de los tipos se aloca en el heap.

Luego se los pasa por referencia.

Si se quiere copia hay que hacerlo explicito:

{{#playpen ./code/heap3.rs}}

Sino... no compila!

{{#playpen ./code/heap2.rs}}

Por que?

Cuando termina el scope de una variable, se la libera de memoria. Si tuvieramos dos punteros a lo mismo, se lo liberaría 2 veces. Rust ataja este problema en tiempo de compilación. Crea el concepto de "ownership" y "move" que veremos en el próximo slide.

## Copy trait

Hace que se creen copias en vez de pasarse por referencia.

## Drop trait

Indica que hacer cuando sale del scope una variable.

Si se implementa copy no se puede implementar drop, y viceversa.


