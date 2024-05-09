// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{}' is '{}.'", s1, len);
// }

// fn calculate_length(s: &String) -> usize {
//     println!("{s}");
//     s.len()
// }

// fn main() {
//     let s = String::from("hello");

//     change(&s); // throws an error because references are immutable by default
// }

// fn change(some_string: &String) {
//     some_string.push_str(", world"); // throws an error because references are immutable bu default
// }

  
// MUTABLE REFERENCES
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}


// Throws an error because rust does create two mutable refrences 
// let mut s = String::from("hello");

// let r1 = &mut s;
// let r2 = &mut s;

// println!("{}, {}", r1, r2);
 
// FIXED ISSUE BY CREATING A NEW SCOPE 

// let mut s = String::from("hello");

//     {
//         let r1 = &mut s;
//     } // r1 goes out of scope here, so we can make a new reference with no problems.

//     let r2 = &mut s;

// let mut s = String::from("hello");

// let r1 = &s; // no problem
// let r2 = &s; // no problem
// let r3 = &mut s; // BIG PROBLEM

// println!("{}, {}, and {}", r1, r2, r3);



// The Rules of References
// Let’s recap what we’ve discussed about references:

// At any given time, you can have either one mutable reference or any number of immutable references.
// References must always be valid.