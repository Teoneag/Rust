// 1. Create a struct called Person with two fields: name and age.
struct Person {
    name: String,
    age: u32,
}

// 4. Add a method to the Person struct that prints out the name and age.
impl Person {
    fn print(&self) {
        println!("{} is {} years old.", self.name, self.age);
    }

    // 5. Add a method to the Person struct that increments the age by 1.
    fn birthday(&mut self) {
        self.age += 1;
    }

    // 6. Add a method to the Person struct that takes ownership of the Person and
    // returns a tuple of the name and age.
    fn to_tuple(self) -> (String, u32) {
        (self.name, self.age)
    }
}

fn main() {
    // 2. Instantiate a Person struct and print out the name and age.
    let p = Person {
        name: String::from("Alice"),
        age: 30,
    };

    // 3. Print out the name and age of the Person struct.
    println!("{} is {} years old.", p.name, p.age);

    // 7. Destructor the Person struct and print out the name and age.
    let Person { name, age } = p;
    println!("{} is {} years old.", name, age);
    // Also works with tuples:
    // let (name, age) = p.to_tuple();
}
