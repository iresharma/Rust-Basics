pub fn run() {
    let mut numbers: [i32; 10] = [0, 1, 2, 3, 4 ,5 ,6 ,7, 8 ,9];
    println!("{:?}", numbers);
    println!("Single Value: {}", numbers[8]);
    numbers[8] = 20;
    println!("Single Value: {}", numbers[8]);
    println!("Len: {}", numbers.len());
    println!("Memory: {}", std::mem::size_of_val(&numbers));
    let slice: &[i32] = &numbers[0..5];
    println!("{:?}", slice);
}