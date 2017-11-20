# Generics

```
fn foo(shape: Shape) {...}

foo(Square { side: 5.0 });
```
No. Para que funcione: ganerics y trait bound.
```
fn foo<T: Shape>(shape: T) {...}
```
* Permite tipos múltiples y trait bounds múltiples.
    ```
    fn bar<T, K>(x: T, y: K) where T: Shape_3D, K: Shape_2D + Clone {...}
    ```
* Aplicable a structs.
    ```
    struct Object<T: 3D_Shape, K: Shape_2D> {
        form: T,
        sprite: K,
    }
    ```