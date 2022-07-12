fn main() {
    /*
        let mut list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);

        let borrows_mutably = |list: &mut Vec<i32>| list.push(7);

        println!("After defining closure: {:?}", list);

        borrows_mutably(&mut list);
        println!("After calling closure: {:?}", list);
    */

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {:?}", list);
}
