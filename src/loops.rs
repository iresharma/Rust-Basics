pub fn run() {
    let mut count = 0;

    loop {
        count += 1;
        println!("Number: {}", count);

        if count == 20 {break;}
    }

    let mut printer: i32 = 0;
    while printer<=100 {
        if printer % 15 == 0 {println!("FizzBuzz")}
        else if printer % 5 == 0 {println!("Fizz")}
        else if printer % 3 == 0 {println!("Buzz")}
        else {println!("{}", printer)}
        printer += 1;
    }

    for x in 0..100 {
        println!("{}", x);
    }
}