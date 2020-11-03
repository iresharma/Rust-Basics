pub fn run() {
    //by default i32
    let _x = 32;
    //by default f64
    let _y = 6.4;
    let _j: i64 = 12345654;
    // Max size
    println!("{}", std::i32::MAX);
    println!("{}", std::i64::MAX);

    let is_greater: bool = 10 > 5;

    println!("{}", is_greater);

    let sd: char = '\u{1f600}';

    println!("{}",sd);
}