fn main() {
    println!("Hello, world!");
    func(5);
    print_labeled_measurement(5, 'h');
    println!("Five Function!{}", five());
    println!("Plus Function!{}", plus_one(6));
}

fn func(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) -> () {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i64 {
    i64::from(x + 1)
    // (x + 1).into()
}
