fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice<T: Fn(i32) -> i32>(f: T, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);

    let answer = do_twice(|x| x + 3, 5);

    println!("The answer is: {}", answer);
}
