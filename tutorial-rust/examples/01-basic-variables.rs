fn main() {
    // Declare a variable
    let x = 10;

    // Print its value
    println!("x = {}", x);
    
    // Try changing the variable
    // x = 7;
    
    // Make it mutable
    let mut x = 10;
    println!("x = {}", x);

    // Try changing the variable again
    x = 7;
    println!("x = {}", x);

    // Try shadowing the variable
    let x = 10;
    let x = x + 1;
    let x = x * 2;
    println!("x = {}", x);
}
