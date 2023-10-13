use std::collections::HashMap;

struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: &str, age: u32) -> Person {
        Person {
            name: String::from(name),
            age: age,
        }
    }

    fn say_hello(&self) {
        println!("Hello, my name is {} and I'm {} years old.", self.name, self.age);
    }
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let mut sum = 0;

    for number in &numbers {
        sum += number;
    }

    println!("The sum of {:?} is {}.", numbers, sum);

    let mut names_and_ages = HashMap::new();
    names_and_ages.insert("Alice", 30);
    names_and_ages.insert("Bob", 25);
    names_and_ages.insert("Charlie", 35);

    for (name, age) in &names_and_ages {
        let person = Person::new(name, *age);
        person.say_hello();
    }
}