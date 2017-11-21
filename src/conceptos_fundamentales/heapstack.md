# Heap, stack, referencias

## Stack

Se alocan los tipos básicos numéricos de tamaño conocido en tiempo de compilación.

* Enteros
* Floats
* Bools
* Tuplas que contengan solo estos tipos

Se pasan por copia.

{{#playpen ./code/heap1.rs}}

## Heap

El resto de los tipos se aloca en el heap.

Luego se los pasa por referencia.

Si se quiere copia hay que hacerlo explicito:

{{#playpen ./code/heap2.rs}}

Si intento usar dos referencias a algo... no compila!

{{#playpen ./code/heap3.rs}}

Por que?

Cuando termina el scope de una variable, se la libera de memoria. Si tuvieramos dos punteros a lo mismo, se lo liberaría 2 veces. Rust ataja este problema en tiempo de compilación. Crea el concepto de "__ownership__", "__move__", y "__borrowing__".

