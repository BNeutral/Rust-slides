# Result

Para errores salvables, un simple struct.

{{#playpen ./code/result1.rs}}

Se usa con pattern matching

{{#playpen ./code/result2.rs}}

## Azucar

Si hay error, panic, sino obtener el valor interior:

{{#playpen ./code/result3.rs}}

Lo mismo pero permite poner un mensaje:

{{#playpen ./code/result4.rs}}

# Propagación de results

Se puede lanzar hacia arriba a mitad de expresión con "return"

{{#playpen ./code/result5.rs}}

## Azucar

Se usa el símbolo "__?__"

{{#playpen ./code/result6.rs}}

El compilador se asegura que "?" solo funciona dentro de funciones que pueden devolver result.

