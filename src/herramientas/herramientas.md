# Herramientas

# Compilador

Compila a ejecutable o a libreria. No existen los header files.

`$ rustc main.rs`

`$ ./main`

# Cargo

Build system y package manager

`$ cargo new hello_world --bin`

Crea una estructura de proyecto, permite especificar y descargar dependencias, incluso inicializa git.

`$ cargo build`

# Crates

Paquetes/librerias de Rust

Actualmente hay 12,425 listos para utilizar en crates.io

## Forma de uso
* Cargo.toml

```
[package]
name = "hola"
version = "0.1.0"
authors = ["Rust <AguanteRust@gmail.com>"]

[dependencies]
rand = "0.3.0"
```
Dependencias por defecto en crates.io, pero se puede especificar la url.

# Documentacion

Se pretende usar sintaxis markdown en comentarios para luego convertirlos en docs html.

`$ cargo doc`

# Tests

Unit tests incluidos via anotaciones en el código.

`$ cargo test`
