fn conditional_function() {
    println!("Condition met!");
}
fn main() {
    #[cfg(feature = "A")]
    conditional_function();
}
