pub fn run() {
    let mut numbers: Vec<i32> = vec![0, 1, 2, 3, 4 ,5 ,6 ,7, 8 ,9];
    println!("{:?}", numbers);
    numbers.push(23);
    println!("Single Value: {}", numbers[8]);
    numbers[8] = 20;
    println!("Single Value: {}", numbers[8]);
    println!("Len: {}", numbers.len());
    println!("{:?}", numbers);
    println!("Memory: {}", std::mem::size_of_val(&numbers));
    let slice: &[i32] = &numbers[0..5];
    println!("{:?}", slice);



    // mutating an ARRAY OR VECTOR

    for x in numbers.iter_mut() {
        *x *= 3;
    }

    println!("{:?}", numbers);
}