pub fn run() {
    greeting("Hello", "Iresh");
    println!("{}", number(3, 7));

    let c: i32 = 23;
    let add_nums = |a: i32, b: i32| a * b + c;

    println!("{}", add_nums(3, 4));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}", greet, name);
}

fn number(a: i32, b: i32) -> i32 {
    // return a + b;
    a + b
}