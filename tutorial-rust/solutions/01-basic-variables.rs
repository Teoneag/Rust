fn main() {
    // Declare a variable
    let x = 10;

    // Print its value
    println!("x = {}", x);

    // Try changing the variable
    x = 15; // error: cannot assign twice to immutable variable `x`

    // Make it mutable
    let mut y = 10;

    // Try changing the variable again
    y = 15; // ok
    println!("y = {}", y);

    // Try shadowing the variable
    let x = 15.2; // shadowing
    println!("x = {}", x);
}
