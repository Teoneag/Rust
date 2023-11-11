fn main() {
    {
        let s = String::from("Hello");
        // The string is owned by `s` and is valid until it goes out of scope.
        println!("s = {}", s);
    }; // <- The String's owner goes out of scope here, so it gets dropped.

    // Try to print `s` here.
    // println!("{}", s); // error: use of undeclared variable `s`

    // Create a String, then move it into a scope and print it. Observe that the String is no longer accessible after the scope ends.
    let s = String::from("Hello");
    {
        // Move `s` into a variable inside the scope
        let inner_s = s;

        // `s` is no longer accessible here
        println!("inner_s = {}", inner_s);
    }

    // Try to print `s` here.
    // println!("{}", s); // error: use of moved value: `s`

    // Declare an integer, then "move" it into a scope and print it. Observe that the integer is still accessible after the scope ends.
    let x = 10;
    {
        // "Move" `x` into a variable inside the scope
        let inner_x = x;
        println!("inner_x = {}", inner_x);
    }
    // `x` is still accessible here
    println!("x outside = {}", x);


    // Create a String, then clone it and move the clone into a scope and print it. Observe that the String is still accessible after the scope ends.
    let s = String::from("Hello");
    {
        // Clone `s` and move the clone into a variable inside the scope
        let inner_s = s.clone();
        println!("inner_s = {}", inner_s);
    }
    // `s` is still accessible here
    println!("s outside = {}", s);

    // Create a String, then pass it to a function and print it. Observe that the String is no longer accessible after the function call.
    let s = String::from("Hello");
    take_ownership(s);
    // println!("s = {}", s); // error: use of moved value: `s`

    // Create a String, then pass it to a function and print the returned String. Observe that the String is still accessible after the function call.
    let s = String::from("Hello");
    let s = take_ownership_and_give_it_back(s);
    println!("s = {}", s);

    // Create a String, drop it with drop(), the try to print it. Observe that the String is no longer accessible after the drop() call.
    let s = String::from("Hello");
    drop(s);
    // println!("s = {}", s); // error: use of moved value: `s`
}

// Write a function that takes ownership of a String and prints it.
fn take_ownership(s: String) {
    println!("take_ownership: s = {}", s);
} // <- `s` goes out of scope and the String is dropped.

// Write a function that takes ownership of a String, appends a word to it, and returns it. Don’t forget to mark the parameter as mutable, like you do with a variable.
fn take_ownership_and_give_it_back(mut s: String) -> String {
    println!("take_ownership_and_give_it_back: s = {}", s);
    s.push_str(", World!");
    s
} // <- `s` goes out of scope and the String is dropped.