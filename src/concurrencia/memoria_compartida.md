#Memoria compartida

##Mutex

- Permiten que los datos sean accedidos por un s�lo thread a la vez.

- Se crean con la funci�n __Mutex::new__ sobre los datos que queremos proteger.

- Para poder acceder a los datos se debe obtener primero un lock sobre ellos mediante la funci�n __lock__ la cual es bloqueante y devuelve una __mutable reference__ a los mismo.

- Al salir de scope, el __mutex__ se libera autom�ticamente.

- Para compartir un __mutex__ entre varios threads, se debe wrappearlo en un __atomic reference counter__ el cual garantiza que las operaciones sean thread safe.

----------------------

##Ejemplo

{{#playpen ./code/mutex.rs}}