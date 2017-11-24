# Threads

- __Rust__ utiliza el modelo conocido como __1:1__ (por cada thread del lenguaje, hay un thread del sistema operativo) dado que tiene menos overhead que el modelo __M:N__.

- Se crean con la función __thread::spawn__ que recibe un __closure__ con el código a ejecutar en el nuevo thread y devuelve un __JoinHandle__ que se puede utilizar para esperar a que terminen todos los threads que se crearon.

- Para poder utilizar datos del thread principal se necesita usar la palabra reservada __move__ antes del closure para que el nuevo thread pueda tomar ownership de esos datos.

----------------------

## Ejemplo

{{#playpen ./code/threads.rs}}
