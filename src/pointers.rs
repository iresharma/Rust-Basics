pub fn run() {
    // Weird thing with non primitive data types

    let vec1: Vec<i32> = vec![1, 2, 3];
    let vec2 = &vec1;

    println!("{:?} {:?}", &vec1, vec2);
}