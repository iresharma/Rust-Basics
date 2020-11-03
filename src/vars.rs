// VARIABLES IN RUST ARE IMMUTABLE BY DEFAULT


pub fn run() {
    let name = "Brad";
    let mut age  = 20;

    println!("Name: {},\nAge: {}", name, age);

    age = 21;

    println!("Name: {},\nAge: {}", name, age);

    const ID: i32 = 00112;

    println!("{}", ID);


    let (my_name, my_age) = ("Iresh", 23);

    println!("Name: {},\nAge: {}", my_name, my_age);
}