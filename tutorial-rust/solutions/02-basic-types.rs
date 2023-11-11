fn main() {
    // Declare a variable with each of these types: i32, u32, f64, bool, char, &str, and print them
    let i: i32 = 42;
    let u: u32 = 42;
    let f: f64 = 3.14;
    let b: bool = true;
    let s: &str = "Rust";
    println!("i = {}, u = {}, f = {}, b = {}, s = {}", i, u, f, b, s);

    // Declare an array of i32 with 3 elements, and print it
    let arr: [i32; 3] = [1, 2, 3];
    println!("arr = {:?}", arr);

    // Print the array's 2nd element
    println!("arr[1] = {}", arr[1]);

    // Make the array mutable using shadowing, and change the 2nd element to 42
    let mut arr: [i32; 3] = [1, 2, 3];
    arr[1] = 42;
    println!("arr = {:?}", arr);

    // Declare a tuple with 3 elements: i32, &str, and f64, and print it
    let tup: (i32, &str, f64) = (1, "Rust", 3.14);
    println!("tup = {:?}", tup);

    // Print its 1st element
    println!("tup.0 = {}", tup.0);
}