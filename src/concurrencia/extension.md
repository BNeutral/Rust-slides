# Extensión

__Rust__ permite crear nuestra propia implementación de concurrencia diferente a la que se encuentra en la __standard library__ utilizando los traits `Send` and `Sync`.


## Send

- Indica que el ownership puede ser transferido a otro thread.

- Casi todos los tipos son `Send` con algunas excepciones.

- Los tipos que están compuestos por tipos `Send`, también lo son.

## Sync

- Indica que el acceso desde múltiples threads es seguro.

- Los tipos que están compuestos por tipos `Sync`, también lo son.