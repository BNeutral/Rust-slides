# Box

Una caja con algo adentro. No tiene overhead. Es de tamaño fijo.

## Casos de uso

* Necesito algo de tamaño fijo, pero mi variable es de tamaño variable. Lo meto en esta caja que es de tamaño fijo.

* Tengo datos y que naturalmente van al stack y se pasan por copia, pero los quiero en el heap por referencia

* Quiero ser dueño de algo con una trait particular, pero no me interesa su tipo.

## Ejemplos

{{#playpen ./code/smbox1.rs}}

❌
{{#playpen ./code/smbox2.rs}}

✔️
{{#playpen ./code/smbox3.rs}}
