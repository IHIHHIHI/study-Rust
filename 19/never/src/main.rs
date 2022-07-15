fn main() {
    println!("Hello, world!");
    bar();
}

fn bar() -> ! {
    loop {}
}
