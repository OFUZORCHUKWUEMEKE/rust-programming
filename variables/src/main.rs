fn main() {
    // Immutable code
    //    let x = 5

    //    println!("The value of x is: {x}")

    //    let y = 5

    //    println!("The value of x is: {y}")
    let mut x = 6;
    // mutability
    x = 7 ;

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    
    println!("My name is: {x} , and i have to achieve a level of success in {THREE_HOURS_IN_SECONDS} time" );

    // SHADOWING
    let y = 5 ;

    let y = y + 4;

    {
        let y = y * 4;
        println!("The value of x in the inner scope is: {y}");
    }
    println!("The value of y is: {y}");
}
