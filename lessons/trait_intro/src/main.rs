// Returns the animal's sound as a String.
trait Speak {
    fn say_hello(&self) -> String;
}

// Implement the Speak trait for different animal types.
struct Dog;
struct Cat;

impl Speak for Dog {
    fn say_hello(&self) -> String {
        String::from("Woof!")
    }
}

impl Speak for Cat {
    fn say_hello(&self) -> String {
        String::from("Meow")
    }
}

/// The main function demonstrates the use of the Speak trait.
fn main() {
    let dog = Dog;
    let cat = Cat;

    println!("Dog says: {}", dog.say_hello());
    println!("Cat says: {}", cat.say_hello());
}