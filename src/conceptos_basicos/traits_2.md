# Traits (2)

* Permiten implementaciones por defecto que pueden ser sobreescritas en `impl`
* Permiten realizar herencia
    ```
    trait Foo {
        fn foo(&self);
    }

    trait Bar : Foo {
        fn bar(&self) {
            self.foo();
        }
    }

    struct Baz;
    impl Foo for Baz {...}
    impl Bar for Baz {
        fn bar(&self) {...}
    }
    ```