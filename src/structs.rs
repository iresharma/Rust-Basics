// Traditional Struct
struct Color {
    red: u8,
    blue: u8,
    green: u8
}

struct Person(String, String, u8);

struct Student {
    first_name: String,
    last_name: String,
    class: u8,
    sec: char
}

impl Student {
    fn new(first: String, last: String) -> Student {
        Student{
            first_name: first,
            last_name: last,
            class: 0,
            sec: 'a'
        }
    }

    fn promote(&mut self, pro: bool) {
        self.class = if pro {self.class + 1} else {self.class}
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn to_tuple(&self) -> (String, u8, char) {
        (self.full_name(), self.class, self.sec)
    }
}

pub fn run() {
    let mut c = Color{
        red: 255,
        blue: 234,
        green: 23
    };

    c.red = 23;

    println!("{} {} {}", c.red, c.green, c.blue);

    let mut _iresh = Person(String::from("Iresh"), String::from("Sharma"), 20);

    println!("{} {} {}", _iresh.0, _iresh.1, _iresh.2);

    let mut iresh: Student = Student::new(String::from("Iresh"), String::from("Sharma"));

    let fullname: String = iresh.full_name();
    println!("{}", fullname);

    println!("{:?}", iresh.to_tuple());
    iresh.promote(true);
    println!("{:?}", iresh.to_tuple());
}