mod my {
    fn private_function() {
        println!("called `my::private_function()`");
    }

    pub fn function() {
        println!("called `my::function()`");
    }

    pub fn indirect_access() {
        print!("called `my::indirect_access()`, that \n> ");
        private_function();
    }

    pub fn indirect_access2() {
        print!("called `my::indirect_access2()`, that \n> ");
        private_nested::function();
    }

    pub mod nested {
        pub fn function() {
            println!("called `my::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `my::nested::private_function()`");
        }
    }

    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `my::private_nested::function()`");
        }
    }
}

fn function() {
    println!("called `function()`");
}

fn main() {
    function();
    my::function();

    my::indirect_access();

    my::indirect_access2();

    my::nested::function();
}
