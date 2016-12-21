fn main() {
    let mut an_integer = 1u32;
    println!("an integer: {:?}", an_integer);

    an_integer = 2u32;

    let a_boolean = true;
    let unit = ();

    let copied_integer = an_integer;

    println!("an integer: {:?}", an_integer);
    println!("an integer: {:?}", copied_integer);
    println!("a boolean: {:?}", a_boolean);
    println!("meet the unit value {:?}", unit);

    let _unused_variable = 3u32;

    // FIXME prefix with an underscore to suppress the warning
    let _noisy_unused_variable = 2u32;
}
