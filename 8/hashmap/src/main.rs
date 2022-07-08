use std::collections::HashMap;
fn main() {
    /*
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("scores: {:?}", scores);
    */

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    println!("scores: {:?}", scores);

    let field_name = String::from("Red");
    let score = 30;
    scores.insert(field_name, score);
    println!("scores: {:?}", scores);
    //    println!("field_name: {}", field_name);
    println!("score: {}", score);

    let field_name = String::from("Red");
    let score = scores.get(&field_name);
    let score = match score {
        Some(score) => *score,
        None => 0,
    };
    println!("score: {}", score);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
