# Canales

- Se utilizan para pasar mensajes entre threads.

- Se crean con la función `mpsc::channel` que devuelve una tupla con un __sender__ y un __receiver__. El __sender__ puede clonarse.

- Para enviar mensajes se usa la función `send` y para recibir mensajes las funciones `try_recv` y `recv`. La última es bloqueante.

- Se puede usar el __receiver__ como un iterador para recibir valores hasta que se cierre el canal.

- Los datos enviados en la función `send` pasan a ser propiedad del __receiver__.

## Ejemplos

{{#playpen ./code/canales1.rs}}

{{#playpen ./code/canales2.rs}}
