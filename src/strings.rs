// There are 2 types of strings in rust
// Primitive str -> Immutatable, fixed length
// String -> Griwable, mutatble

pub fn run() {
    // Primitive str
    let _x = "Iresh";

    // String Object
    let mut name = String::from("Hello");

    
    println!("{}, {}", name, name.len());
    name.push(' ');
    println!("{}, {}", name, name.len());
    name.push_str("World");
    println!("{}, {}\nCapacity: {}", name, name.len(), name.capacity());
    let empty = String::from("");
    println!("{}", empty.is_empty());

    //  Contains and replace
    // works as normal on the String

    for word in name.split_whitespace() {
        println!("{}", word);
    }
}