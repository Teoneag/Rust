fn main() {
    // Declare a variable named `x` 
    let x = 3;

    // Create a scope and declare a variable named `y` in it. Print `x + y` from the scope.
    {
        let y = 4;
        println!("x + y = {}", x + y);
    };

    // Create a scope that returns `x + y` from it, assign it to a variable named `z` and print it.
    let z = {
        let y = 4;
        x + y
    };
    println!("z = {}", z);

    // Use if/else to print whether z is greater than 1 or not.
    if z > 1 {
        println!("z is greater than 1");
    } else {
        println!("z is not greater than 1");
    }

    // Use a `loop` to print `z` 5 times.
    let mut i = 0;
    loop {
        println!("z = {}", z);
        i += 1;
        if i == 5 {
            break;
        }
    }

    // Use a while loop to print `z` 5 times (use an additional counter variable).
    let mut i = 0;
    while i < 5 {
        println!("z = {}", z);
        i += 1;
    }

    // Use if else to declare a variable that is true if `z` is greater than 1 and false otherwise.
    let is_greater_than_1 = if z > 1 {
        true
    } else {
        false
    };
    println!("is_greater_than_1 = {}", is_greater_than_1);

    // Use a loop that returns the first randomly generated value that is greater than 0.5 (use the function `rand::random::<f64>()` to get a random float between 0 and 1).
    let random_value = loop {
        let a = rand::random::<f64>();
        if a > 0.5 {
            break a;
        }
    };
    println!("random_value = {}", random_value);

    // Use a for loop to print the numbers 1 to 10.
    for i in 1..11 {
        println!("i = {}", i);
    }

    // Use a for loop to iterate throuth and array
    let arr = [1, 2, 3, 4, 5];
    for i in arr {
        println!("i = {}", i);
    }
}
