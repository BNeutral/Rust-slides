trait Supertrait {
}

trait HasSupertrait: Supertrait {
}


fn main() {
    struct SomeStruct {
    }

    impl HasSupertrait for SomeStruct {
    }
    impl Supertrait for SomeStruct {
    }
}