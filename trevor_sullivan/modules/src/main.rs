pub mod helpers;

fn main() {
    println!("Hello, world!");
    let myresult = helpers::namehelpers::get_full_name("Chukwuemeke","Ofuzor");
    println!("{}",myresult)
}


