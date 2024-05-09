fn main() {
    // datatypes in the rust programming language

    // u8 , f32 , f64
    println!("Hello, world!");

    //  floating point
    // f64
    let x = 2.0;

    // f32
    let y: f32 = 3.0;

    // addition
    let sum = 5 + 10;

    // substraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.3;
    let truncated = -5 / 3;

    // remainder
    let remainder = 43 % 5;

    // boolean
    let t = true;

    let y: bool = false;

    // char
    let c = 'z';
    let z: char = 'Z';

    // Tuple
    let tup: (i32, f64, u8) = (500, 5.6, 1);

    let tuplee = (500, 6.4, 1);

    let (x, y, z) = tuplee;

    println!("The value of y is:{y}");

    let p: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = p.0;

    let six_point_four = p.1;

    let one = x.2;

    println!(five_hundred, six_point_four);

    let a = [1, 2, 3, 4];

    let months = ["january", "february", "march", "April", "May", "June"];

    let ab: [i32; 5] = [1, 2, 3, 4, 5];

    let first = ab[0];

    let second = ab[1];
}
