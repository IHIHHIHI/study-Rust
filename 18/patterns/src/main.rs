struct St {
    x: String,
    y: i32,
}

struct Point {
    x: i32,
    y: i32,
}
fn main() {
    let p = St {
        x: String::from("asdf"),
        y: 7,
    };

    let St { x: a, y: b } = p;
    println!("{}", a);
    println!("{}", b);
    //    println!("{}", p.x);
    println!("{}", p.y);
    let x = 5;
    match x {
        1..=5 => println!("1 ~ 5"),
        _ => println!("something else"),
    };
    println!("Hello, world!");
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}
