use std::fmt;

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;
    (boolean, integer)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Matrix(a, b, c, d) = *self;
        write!(f, "({}, {})\n({}, {})", a, b, c, d)
    }
}

fn main() {
    let long_tuple = (1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true);
    println!("long tuple first elem {}", long_tuple.0);
    println!("long tuple second elem {}", long_tuple.1);

    let tuple_of_tuples = ((1u8, 2u16, 3u32), (4f32, 8f64), -2i16);
    println!("tuple of tuples {:?}", tuple_of_tuples);

    println!("one element tuple {:?}", (1,));
    println!("just a integer {}", (1));

    let pair = (1, true);
    println!("pair = {:?}, pair reverse = {:?}", pair, reverse(pair));


    let tuple = (8u8, 15i32, true, "hello");
    let (a, b, c, d) = tuple;
    println!("a = {:?}, b = {:?}, c = {:?}, d = {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.1);
    println!("matrix = {:?}", matrix);
    println!("matrix = {}", matrix);
}
