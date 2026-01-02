fn main() {
    // OK
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // OK Shadowing
    let y = 10;
    println!("The value of y is: {y}");
    let y = 15;
    println!("The value of y is: {y}");

    // Warning
    let a = 10;
    println!("The value of a is: {a}");
    let a = 5;
    println!("The value of a is: {a}");

    // Not OK Compile Error
    let z = 20;
    println!("The value of z is: {z}");
    z = 25;
    println!("The value of z is: {z}");

    // Not OK Compile Error
    let mut n = 30;
    println!("The value of n is: {n}");
    n = "35";
    println!("The value of n is: {n}");
}