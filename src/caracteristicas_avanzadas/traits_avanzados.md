# Traits avanzados

## Associated Types

- Son placeholders de tipos en la definición de un trait.

- Se definen con la palabra reservada `type`.

### Ejemplo

{{#playpen ./code/asociated_types.rs}}

---

## Sobrecarga de operadores

- __Rust__ no permite crear ni sobrecargar operadores pero provee operaciones en la librería estándar que si pueden ser sobrecargadas implementando los traits asociados con el operador.

### Ejemplo

{{#playpen ./code/operator_overloading.rs}}

---

## Fully Qualified Syntax

### Ejemplo

{{#playpen ./code/fully_qualified_syntax.rs}}

---

## Supertraits

- Son traits requeridos por el trait que se está implementando.

### Ejemplo

{{#playpen ./code/supertraits.rs}}

---

## Newtype pattern

- __Rust__ solo permite implementar traits siempre y cuando el trait o el tipo sean locales a nuestro crate.

- Este pattern se puede utilizar para evitar esa restricción.

### Ejemplo

{{#playpen ./code/newtype_pattern.rs}}