enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn move_player(num_spaces: u8) {}

fn value_in_cents(coin:Coin)->u8{
    match coin{
        Coin::Penny => {
            println!("Lucky Penny!");
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter =>25
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}


fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}


fn main() {
    println!("Hello, world!");
}
