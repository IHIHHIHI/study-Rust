use std::collections::HashMap;
use std::io;
fn readint() -> i32 {
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("failed to read line.");
    num.trim().parse().expect("input an integer")
}
fn main() {
    let n = readint();
    let mut nums: Vec<i32> = Vec::new();
    let mut occur = HashMap::new();
    for _ in 0..n {
        let num = readint();
        nums.push(num);
        let count = occur.entry(num).or_insert(0);
        *count += 1;
    }
    nums.sort();
    println!("median: {}", nums[(n / 2) as usize]);
    let mut mx = 0;
    let mut mxval = 0;
    for (key, val) in occur {
        if val > mxval {
            mxval = val;
            mx = key;
        }
    }
    println!("mode: {}", mx);
}
