fn foo() {
    let optional = Some(7);

    match optional {
        Some(i) => {
            println!("This is a really long string and {:?}", i);
        }
        _ => {}
    };
}

fn bar() {
    let number = Some(7);
    let letter: Option<i32> = None;
    let emotion: Option<i32> = None;

    if let Some(i) = number {
        println!("Matched {:?}", i);
    }
    if let Some(i) = letter {
        println!("Matched {:?}", i);
    } else {
        println!("Didn't match a number, let's go with a letter");
    };

    let i_like_letters = false;

    if let Some(i) = emotion {
        println!("Matched {:?}", i);
    } else if i_like_letters {
        println!("Didn't match a number, let's go with a letter");
    } else {
        println!("I don't like letters, let's go with an emotion :)");
    };
}

fn main() {
    foo();
    bar();
}
