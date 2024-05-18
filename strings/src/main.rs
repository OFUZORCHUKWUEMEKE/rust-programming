// Creating a New String

fn main() {
    let mut s = String::new();

    let data = "initial contents";

    // let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    println!("{s}");

    let mut y = String::from("foo");
    y.push_str("bar");

    println!("{y}");

    // Appending to a String with push_str and push

    // let s1 = String::from("Hello, ");
    // let s2 = String::from("world!");
    // let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");

    // let s = format!("{s1}-{s2}-{s3}");


    let s1 = String::from("hello");
    let h = s1[0];


    // for c in "Зд".chars() {
    //     println!("{c}");
    // }
    
}
