fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{s}");
    let s2: String;
    let n1: u32;
    if 1 == 1 {
        let s1 = String::from("hello");
        s2 = s1;
    } else {
        let s1 = String::from("hello");
        s2 = s1;
    }
    n1 = 12;
    println!("{n1}");

    println!("{}, world!", s2);
}
