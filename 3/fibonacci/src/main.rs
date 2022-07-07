use std::io;
fn main() {
    println!("Hello, world!");
    let mut a = 1;
    let mut b = 1;
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line.");
    let n: u32 = n.trim().parse().expect("Not a number.");
    for _ in 0..n - 1 {
        let c = a + b;
        a = b;
        b = c;
    }
    println!("The nth fibonacci number is {a}");
}
