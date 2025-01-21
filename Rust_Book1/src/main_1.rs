fn main() {
    println!("Variables & Mutability");
    //---- When a variable is immutable by default,it means you cannot change the value of the variable
    println!("--------------------");
    let mut x = 5; //let mut x = 5; (makes the value of the variable to changed)
    println!("x: {x}");

    x = 6;
    println!("x: {x}");
    /*
    From the above You received the error message cannot assign twice to immutable variable `x`
    because you tried to assign a second value to the immutable x variable.
     */

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("3 hours in seconds: {THREE_HOURS_IN_SECONDS}");
    //Rust’s naming convention for constants is to use all uppercase with underscores between words

    //Shadowing - allows us to resue a variable name with different data types
    let spaces = "  ";
    let _spaces = spaces.len();

    println!("DATA TYPES - SCALAR AND COMPOUND");
    //Scalar - interger, float, boolean, character
    //interger has i(signed), u(unsigned) - i(from -(2^n -1) to 2^n - 1)
    //u from 0 to 2^n -1
    let x: i8 = 120;
    println!("x: integer {x}");

    //floating point - f32(single precision), f64 (double precision)
    let _x = 2.0; //f64 by default
    let y: f32 = 3.0;
    println!("y: floating point {y}");

    //Boolean- default is true
    let _t = true;
    let f: bool = false;
    println!("f: boolean {f}");

    //Char
    let c = 'z';
    let _z: char = 'Z';
    println!("c:character {c}");

    //COMPOUND DATA TYPES - Tuples and Arrays
    //Tuple - are a grouped of data but of different data types and cannot shrink or grow
    let tup: (i32, f64, u8) = (500, 64.0, 1);
    let (x, y, z) = tup;
    println!("The value of z is {z}");

    //we can also access tuple by using a period followed by index of the value
    let one = tup.1;
    println!("The value of one: {one}");

    //ARRAYS
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [6, 7, 8, 9, 10];
    /*
    Here the i32 is the type of element. After the semicolon, the number 5 indicates
    the array contains 5 elements.
     */

    //printing same numbers muiltiple times
    let c = [2; 4];

    //Accessing Array Elements
    let first_of_a = a[0];
    let third_of_a = a[3];
    println!("index 1 of a: {first_of_a}");
    println!("index 2 of a: {third_of_a}");

    //Functions
    another_function(); //calling functions
    another_function2(200); //calling functions with value in it
    multiple_parameters(6, 'w'); //multiple parameters

    //Functions with Return Values
    let return_values = five();
    println!("The value of x is {return_values}");

    let plus_one = plus_one(6);
    println!("Return value of plus one is {plus_one}");
}

fn another_function() {
    println!("Another Function Defined outisde of the main function");
}

/*
Parameters: this are special variables that are part of a function signature.
This values are called arguments.
 */
fn another_function2(x: i32) {
    println!("The value of x is: {x}");
}

fn multiple_parameters(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

/*
Functions with return values
----------------------------
- Function can return values to the code that calls them. We don't name return values, but
we must declare their type after an arrow (->).
- When dealing with expression you do not put a semicolon
*/

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
fn main() {
    println!("Variables & Mutability");
    //---- When a variable is immutable by default,it means you cannot change the value of the variable
    println!("--------------------");
    let mut x = 5; //let mut x = 5; (makes the value of the variable to changed)
    println!("x: {x}");

    x = 6;
    println!("x: {x}");
    /*
    From the above You received the error message cannot assign twice to immutable variable `x`
    because you tried to assign a second value to the immutable x variable.
     */

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("3 hours in seconds: {THREE_HOURS_IN_SECONDS}");
    //Rust’s naming convention for constants is to use all uppercase with underscores between words

    //Shadowing - allows us to resue a variable name with different data types
    let spaces = "  ";
    let _spaces = spaces.len();

    println!("DATA TYPES - SCALAR AND COMPOUND");
    //Scalar - interger, float, boolean, character
    //interger has i(signed), u(unsigned) - i(from -(2^n -1) to 2^n - 1)
    //u from 0 to 2^n -1
    let x: i8 = 120;
    println!("x: integer {x}");

    //floating point - f32(single precision), f64 (double precision)
    let _x = 2.0; //f64 by default
    let y: f32 = 3.0;
    println!("y: floating point {y}");

    //Boolean- default is true
    let _t = true;
    let f: bool = false;
    println!("f: boolean {f}");

    //Char
    let c = 'z';
    let _z: char = 'Z';
    println!("c:character {c}");

    //COMPOUND DATA TYPES - Tuples and Arrays
    //Tuple - are a grouped of data but of different data types and cannot shrink or grow
    let tup: (i32, f64, u8) = (500, 64.0, 1);
    let (x, y, z) = tup;
    println!("The value of z is {z}");

    //we can also access tuple by using a period followed by index of the value
    let one = tup.1;
    println!("The value of one: {one}");

    //ARRAYS
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [6, 7, 8, 9, 10];
    /*
    Here the i32 is the type of element. After the semicolon, the number 5 indicates
    the array contains 5 elements.
     */

    //printing same numbers muiltiple times
    let c = [2; 4];

    //Accessing Array Elements
    let first_of_a = a[0];
    let third_of_a = a[3];
    println!("index 1 of a: {first_of_a}");
    println!("index 2 of a: {third_of_a}");

    //Functions
    another_function(); //calling functions
    another_function2(200); //calling functions with value in it
    multiple_parameters(6, 'w'); //multiple parameters

    //Functions with Return Values
    let return_values = five();
    println!("The value of x is {return_values}");

    let plus_one = plus_one(6);
    println!("Return value of plus one is {plus_one}");
}

fn another_function() {
    println!("Another Function Defined outisde of the main function");
}

/*
Parameters: this are special variables that are part of a function signature.
This values are called arguments.
 */
fn another_function2(x: i32) {
    println!("The value of x is: {x}");
}

fn multiple_parameters(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

/*
Functions with return values
----------------------------
- Function can return values to the code that calls them. We don't name return values, but
we must declare their type after an arrow (->).
- When dealing with expression you do not put a semicolon
*/

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
