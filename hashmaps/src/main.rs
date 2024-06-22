use std::collections::HashMap;



fn main() {


    // CREATING A NEW HASH MAP

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("yellow"),50);
    println!("Hello, world!");

    // ACCESSING VALUES IN A HASH MAP
    
    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("yellow"),50);


    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    let team_name = String::from("Blue");

    let score = scores.get(&team_name).copied().unwrap_or();


    for (key,value) in &scores{
        println!("{key}: {value}")
    }
}
