# Rc: Reference counted smart pointer

Permite multiples owners, pero solo inmutablemente.

Clonar el puntero copia la referencia interna e incrementa el contador de referencias.

Cuando las referencias se vuelven 0, se borra el contenido.

✔️
{{#playpen ./code/smrc1.rs}}

❌
{{#playpen ./code/smrc2.rs}}
