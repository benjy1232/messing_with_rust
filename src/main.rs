pub mod structures;

fn main() {
    println!("Hello, world!");
    let benjy = structures::Person {
        name: String::from("Benjy"),
        age: 19,
    };
    println!("name: {}\nage: {}", benjy.name, benjy.age);
}
