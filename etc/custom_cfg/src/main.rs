fn conditional_function() {
    #[cfg(some_condition)]
    println!("Condition met!");
}
fn main() {
    conditional_function();
}
