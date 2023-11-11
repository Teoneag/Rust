fn main() {
    // Create a mutable String with `String::from` and print it.
    let mut s = String::from("Hello");
    println!("{}", s);

    // Add ", World!" to the String and print it.
    s.push_str(", World!");
    println!("{}", s);
}