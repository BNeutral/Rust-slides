#Memoria compartida

##Mutex

- Permiten que los datos sean accedidos por un sólo thread a la vez.

- Se crean con la función __Mutex::new__ sobre los datos que queremos proteger.

- Para poder acceder a los datos se debe obtener primero un lock sobre ellos mediante la función __lock__ la cual es bloqueante y devuelve una __mutable reference__ a los mismo.

- Al salir de scope, el __mutex__ se libera automáticamente.

- Para compartir un __mutex__ entre varios threads, se debe wrappearlo en un __atomic reference counter__ el cual garantiza que las operaciones sean thread safe.

----------------------

##Ejemplo

{{#playpen ./code/mutex.rs}}