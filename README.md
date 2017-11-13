# Rust slides

Presentación introductoria sobre lenguaje de programación Rust

# Dependencias

cargo install mdbook

Luego agregar a path para ejecutar mdbook

# Buildear

mdbook build

Se crea el libro en ./book

Se abre el resultado desde index.html

# Sintaxis especifica

Markdown típico, varias referencias por la web.

Para insertar codigo rust se puede hacer 

{{#playpen ./ruta al archivo.rs}}

Aquellas lineas que tengan # adelante no se mostraran.

O alternativamente

\`\`\`rust
//Codigo de rust
\`\`\`
