pub fn run() {
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);

    let command: String = args[1].clone();

    // println!("Command: {}", command);

    if command == "Hello" {
        println!("What\'s up bitch");
    } else {
        println!("Yes");
    }
}