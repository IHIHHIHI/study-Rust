use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let num1 = Arc::new(Mutex::new(0));
    let num2 = Arc::new(Mutex::new(0));
    let num11 = Arc::clone(&num1);
    let num12 = Arc::clone(&num2);
    let num21 = Arc::clone(&num1);
    let num22 = Arc::clone(&num2);
    let handle1 = thread::spawn(move || {
        let mut val1 = num11.lock().unwrap();
        thread::sleep(Duration::from_secs(1));
        let val2 = num12.lock().unwrap();
        *val1 += *val2;
    });
    let handle2 = thread::spawn(move || {
        let val2 = num22.lock().unwrap();
        thread::sleep(Duration::from_secs(1));
        let mut val1 = num21.lock().unwrap();
        *val1 += *val2;
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
}
