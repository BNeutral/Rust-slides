# Deref y drop

Traits especiales de Rust.

# Deref

Permite especificar que obtenemos al desreferenciar el struct, por lo cual podemos hacer que se comporte como una referencia común.

Ademas realiza coerciones automáticas cuando las funciones y demas esperan referencias.

# DerefMut

Lo mismo pero para cuando se esperan referencias mutables.

# Drop

Especifica que tiene que hacer el struct cuando sale del scope. 

Casos tipicos: Reference counting, struct que representa un archivo, struct que representa un mutex.

Si por alguna razon queremos llamar a drop antes que lo haga el lenguaje naturalmente, esta permitido via std::mem::drop.
