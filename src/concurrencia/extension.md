# Extensión

__Rust__ permite crear nuestra propia implementación de concurrencia diferente a la que se encuentra en la __standard library__ utilizando los traits __Send__ and __Sync__.


## Send

- Indica que el ownership puede ser transferido a otro thread.

- Casi todos los tipos son __Send__ con algunas excepciones.

- Los tipos que están compuestos por tipos __Send__, también lo son.

## Sync

-  Indica que el acceso desde múltiples threads es seguro.

- Los tipos que están compuestos por tipos __Sync__, también lo son.
