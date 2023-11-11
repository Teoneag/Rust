// 1. Define a function that takes a name as string literal as an argument and prints a greeting in your native language.
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

// 3. Define a function that takes a number as an argument and returns the square of that number.
fn square(x: i32) -> i32 {
    x * x
}

fn main() {
    // 2. Call your greeting function with a string literal as an argument.
    greet("Rustacean");

    // 4. Call your square function with a number as an argument and print the result.
    println!("square(2) = {}", square(2));
}