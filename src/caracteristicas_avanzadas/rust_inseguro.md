# Rust inseguro

- __Rust__ permite utilizar código considerado inseguro, delegando la responsabilidad en el programador, dentro de un bloque definido con la palabra reservada `unsafe`.

- Dentro de un bloque marcado como inseguro, el compilador no realiza chequeos de __memory safety__ lo que permite ciertas libertades.

## Desreferenciado de raw pointers

- Si bien la creación de estos punteros es válida fuera de un bloque inseguro, el acceso a la información a la que apunta no lo es.

---

## Funciones inseguras

- Funciones que tienen en su interior código inseguro.

- Se declaran agregando `unsafe` antes de la definición de la misma.

---

## Abstracciones seguras con código inseguro

- Casos en que el compilador, por ser conservador, no permite realizar una operación que sería válida.

### Ejemplo
{{#playpen ./code/safe_over_unsafe.rs}}

---

## Llamado de código externo

- Llamar código escrito en otro lenguaje es, para __Rust__ inseguro por definición.

---

## Acceso o modificación de variable estática mutable 

- __Rust__ lo considera inseguro dado que pueden ser accedidas por varios threads lo que puede causar condiciones de carrera.

---

## Implementación de traits inseguros

- Similar a las __unsafe functions__.

- Se declaran agregando `unsafe` antes de la definición y de la implementación del trait.