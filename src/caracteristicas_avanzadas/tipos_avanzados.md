# Tipos avanzados

## Type aliases

- __Rust__ permite definir alias para los tipos de la siguiente forma `type alias = tipo`

---

## The Never type

- En __Rust__ existe el tipo `!` llamado __Never type__.

- Se utiliza para indicar que una función nunca retorna.

### Ejemplo

{{#playpen ./code/never_type.rs}}

---

## Trait Sized

- Por defecto los generics aceptan sólo tipos para los cuales se conoce en tiempo de compilación el tamaño que ocupan en memoria.

- Para indicar al compilador que se quiere utilizar un __usized type__, se debe agregar la sintaxis ` <T: ?Sized>`

### Ejemplo

{{#playpen ./code/sized_trait.rs}}