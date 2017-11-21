# Extensi�n

__Rust__ permite crear nuestra propia implementaci�n de concurrencia diferente a la que se encuentra en la __standard library__ utilizando los traits __Send__ and __Sync__.


## Send

- Indica que el ownership puede ser transferido a otro thread.

- Casi todos los tipos son __Send__ con algunas excepciones.

- Los tipos que est�n compuestos por tipos __Send__, tambi�n lo son.

## Sync

-  Indica que el acceso desde m�ltiples threads es seguro.

- Los tipos que est�n compuestos por tipos __Sync__, tambi�n lo son.