enum Movement {
    Left,
    Right,
    Up,
    Dowm
}

fn moves(m: Movement) {
    match m {
        Movement::Up => println!("Moving Up"),
        Movement::Dowm => println!("Moving Down"),
        Movement::Left => println!("Moving Left"),
        Movement::Right => println!("Moving Right"),
    }
}

pub fn run() {
    let avatar = Movement::Right;
    moves(avatar);
}