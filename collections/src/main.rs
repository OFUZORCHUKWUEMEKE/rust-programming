Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory.

fn main() {

    let u = vec![1, 2, 3];

    let v:Vec<i32> = Vec::new()

    // Updating a Vector

    let mut y = Vec::new();


    y.push(5);

    y.push(6);

    y.push(7);

    y.push(8);
    
    println!("Hello, world!");

    let x = vec![1,2,3,4,5];

    let third:Option<&i32> = v.get(2);

    match third{
        Some(third)=> println!("The third element is {third}")
        None=> printin!("There is so third element.")
    }

    let y = vec![100, 32, 57];
    for i in &y {
        println!("{i}");
    }

    let mut z = vec![100,32,57];

    for i in &mut v{
        *i += 50;
    }

    // Using an Enum to Store Multiple Types
    // Vectors can only store values that are the same type. This can be inconvenient; there are differently use cases for needng to store a list of items of different types.


    emum SpreadsheetCell{
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12)
    ];

    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here
}
