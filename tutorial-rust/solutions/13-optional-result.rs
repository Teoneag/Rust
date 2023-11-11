// 1. Write a function that takes a string and finds the index of a word in a string.
fn find_word(string: &str, word: &str) -> Option<usize> {
    for (i, w) in string.split_whitespace().enumerate() {
        if w == word {
            return Some(i);
        }
    }
    None
}

// 5. Write a function that lists files in "data/test2" and returns the contents of the first file
fn list_files_and_read_first() -> Result<String, std::io::Error> {
    let files = std::fs::read_dir("data/test2")?;
    for file in files {
        let file = file?;
        let content = std::fs::read_to_string(file.path())?;
        return Ok(content);
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "No files found",
    ))
}

fn main() {
    // 2. Call the function with a string and a word that is in the string.
    if let Some(i) = find_word("hello world", "world") {
        println!("Found word at index {}", i);
    }

    // 3. Use the standard library to load a file into a string.
    let content = std::fs::read_to_string("data/test1.txt");
    match content {
        Ok(content) => {
            // 4. Call the function with the string and a word that is in the string.
            if let Some(i) = find_word(&content, "hello") {
                println!("Found word at index {}", i);
            }
        }
        Err(error) => println!("Error: {}", error),
    }

    // 6. Call the list_files_and_read_first function and print the contents of the file.
    match list_files_and_read_first() {
        Ok(content) => println!("Content: {}", content),
        Err(error) => println!("Error: {}", error),
    }

    // 7. We can use a use statement to bring std into scope
}
