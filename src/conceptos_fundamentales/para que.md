# Para que esto de ownership y lifetimes?

* Manejo de memoria "automático" sin GC (RAII forzado)

* No permite cosas inseguras en memoria: Usar cosas sin inicializar, no hay overflows/dangling pointers/segfaults.

* Hace dificil tener memory leaks, aunque no imposible.

* Evita data races.

Basicamente el código es seguro en tiempo de compilación.

Pero todo esto es un poco restrictivo.
