# Tiempos de vida avanzados

## Lifetime Subtyping

- Si el lifetime `'b` contiene `'a`, entonces `'b` es subtipo de `'a`.

- Se declara de la forma `'b: 'a`.

### Ejemplos

{{#playpen ./code/lifetime_subtyping.rs}}

---

## Lifetime Bounds

- Son restricciones para los lifetimes de los tipos genéricos.

- Se declaran de la misma forma que en __lifetime subtyping__.

### Ejemplos

{{#playpen ./code/lifetime_bounds.rs}}