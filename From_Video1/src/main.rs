fn main() {
    let x = 4; //declaring a variable
    println!("x is: {x}");

    let x = "Daniel";
    println!("x is: {x}");

    //declaring constants
    const SECONDS_IN_MINUTE: u32 = 60;
    println!("This number is constant: {SECONDS_IN_MINUTE}");

    //Data Types
    //Int, Uint
    let x: i8 = -70; //i8 (-128 o 127)
    let y: u8 = 100; //u8 (0 - 255)
    println!("x: {x} and y: {y}");

    //Floating Points
    //f32 - single precision
    //f64 - doube precision
    let floating_point: f32 = 10.9;
    let floating_point2: f64 = 20.87;
    println!("single precision: {floating_point} double precision: {floating_point2}");

    //boolean
    let true_or_false: bool = true;
    println!("bool: {true_or_false}");

    //characters:
    let letter: char = 'd';
    println!("character: {letter}");

    //Compound Data Types, tuple and array
    let mut tup = (1, true, 's');
    println!("value of tuple: {}", tup.0);

    let tup = (2, 'd', "Daniel");
    println!("value of tuple2: {}", tup.2);
}
