fn main() {
    println!("Hello, world!");
    // testif()
    // testWhile()
    testFor()
}

fn testif() {
    let age_to_drive: u8 = 15;

    println!("Enter the person's age:");
    let myinput: &mut String = &mut String::from("");
    std::io::stdin().read_line(myinput).unwrap();

    let age: u8 = myinput.replace("\n", "").parse::<u8>().unwrap();
    if age >= age_to_drive {
        println!("Issuing drivers license , because they are old enough");
    }
}

fn testWhile() {
    let age_to_drive = 16u8;

    let mut current_age = 0u8;

    while current_age < age_to_drive {
        println!("Waiting....");
        current_age += 1;
    }
}

fn testFor() {
    const ages: [i32; 5] = [2, 3, 5, 6, 8];
    for age in &ages {
        println!("waiting")
    }
}
