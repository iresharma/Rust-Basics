pub fn run() {
    //print to console
    println!("Hello World from print.rs");
    //basic formatting
    println!("Printing numbers is c type {}", 1);
    //positioned formating
    println!("{0} first {1} second {0} first {2} third", "Brad", "Irsh", 3);
    //named formatting
    println!("My name is {Name}, i like to {like}", Name = "Iresh", like = "code");
    //converters
    println!("Binary {:b}, Hex {:x}, oct {:0}", 2, 1, 1);
    //debug trait
    println!("{:?}", (1, true, 123, "hierh", 12/4));
}
