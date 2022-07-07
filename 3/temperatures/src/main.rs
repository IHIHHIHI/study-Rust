fn main() {
    println!("Hello, world!");
    println!("Farenheit 212 : celsius {}", farenheit_to_celsius(212.0));
    println!("celsius 1000 : Farenheit {}", celsius_to_farenheit(1000.0));
}

fn farenheit_to_celsius(farenheit: f64) -> f64 {
    (farenheit - 32.0) * 5.0 / 9.0
}
fn celsius_to_farenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}
