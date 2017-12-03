fn main() {
    fn generic_function<T>(t: T) {
    }

    fn actual_generic_function<T: Sized>(t: T) {
    }

    fn dinamically_sized_generic_function<T: ?Sized>(t: &T) {
    }
}