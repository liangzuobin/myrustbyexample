fn main() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("before mutation: {}", mutable_binding);

    // OK
    mutable_binding += 1;

    println!("after mutatio: {}", mutable_binding);

    // _immutable_binding += 1;
}
