// 1. Define an enum called Message with two variants: Quit and Move.
enum Message {
    Quit,
    // 3. Add data to the Move variant that holds an x and y coordinate as i32s.
    Move { x: i32, y: i32 },
    // 6. Add a Write variant that holds a String, and see the error that occurs.
    Write(String),
}

fn main() {
    // 2. Instantiate a Message enum
    let m = Message::Quit;
    
    // 4. Instantiate a Message enum with the Move variant
    let m = Message::Move { x: 3, y: 4 };

    // 5. Match on the Message enum and print out the x and y coordinates if the
    // variant is Move.
    match m {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        // 7. Handle the Write variant by printing out the String.
        Message::Write(ref s) => println!("Write {}", s),
        // 8. Add a catch-all arm to the match to handle the Write variant.
        _ => println!("Other"),
    }

    // 9. Use if let to handle the Write variant.
    if let Message::Write(s) = m {
        println!("Write {}", s);
    }
}
