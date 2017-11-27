# Weak

Si usamos __Rc__, es posible crear referencias circulares que nunca se liberan.

Leakear memoria es indeseable, pero permitido por ser memory safe.

El __Weak__ es un derivado del __Rc__ que no aumenta el contador, no toma ownership.

Sin embargo, no podría esto causar dangling pointers? El weak devuelve un Option.

{{#playpen ./code/smweak1.rs}}
