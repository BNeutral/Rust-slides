# Canales

- Se utilizan para pasar mensajes entre threads.

- Se crean con la funci�n __mpsc::channel__ que devuelve una tupla con un __sender__ y un __receiver__. El __sender__ puede clonarse.

- Para enviar mensajes se usa la funci�n __send__ y para recibir mensajes las funciones __try_recv__ y __recv__. La �ltima es bloqueante.

- Se puede usar el __receiver__ como un iterador para recibir valores hasta que se cierre el canal.

- Los datos enviados en la funci�n __send__ pasan a ser propiedad del __receiver__.

----------------------

## Ejemplos

{{#playpen ./code/canales1.rs}}

________________________________

{{#playpen ./code/canales2.rs}}