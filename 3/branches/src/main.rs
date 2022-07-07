fn main() {
    let number = 3;

    let itval: () = if number < 5 {
        println!("True!!");
    } else {
        println!("False..");
    };

    let number = if itval == () { 5 } else { 6 };
    println!("number : {number}");
}
