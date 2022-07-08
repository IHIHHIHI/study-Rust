fn main() {
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    let s = "initial contents".to_string();

    let mut s = String::from("안녕하세요.");

    println!("{}", s);

    let mut s2 = "hihihi~";

    s.push_str(s2);
    println!(" s2 is {}", s2);
    println!(" s is {}", s);

    let s3 = s + &s2;
    println!(" s3 is {}", s3);

    let s4 = format!("{}-{}", s2, s3);
    println!("{}", s4);
}
