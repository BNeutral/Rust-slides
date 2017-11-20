# Null?

__No existe__

Hay __tipo__ unit () que representa nada, pero es un tipo, no un valor. Similar a un void.

✔️
{{#playpen ./code/unit1.rs}}

❌
{{#playpen ./code/unit2.rs}}

❌ por no inicializado 
{{#playpen ./code/unit3.rs}}

# Option

Similar al maybe de otros lenguajes, tiene algo o no tiene nada.

Es simplemente un enum.

{{#playpen ./code/option1.rs}}

Se usa pattern matching para leer los resultados.

{{#playpen ./code/option2.rs}}

Existe azucar para cuando no hago nada si es None.

{{#playpen ./code/option3.rs}}
