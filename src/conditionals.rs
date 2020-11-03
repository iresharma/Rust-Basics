pub fn run() {
    let age: i32 = 21;

    if age >= 21{
        println!("Hi, What do you wanna drink");
    } else {
        println!("Sorry you have to leave");
    }

    let Vote: &str = if age > 18 { "Yes you can vote" } else { "Sorry you can't vote" };

    println!("{}", Vote);
}