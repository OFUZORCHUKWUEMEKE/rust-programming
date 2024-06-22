fn main() {
    let user = User {
        name: true,
        username: "Emeke001".to_string(),
        email: "uniben2018@gmail.com".to_string(),
        sign_in_count: 19,
    };
    println!("{:?}", user);
}
enum VehicleColor {
    Silver,
    Blue,
    Red,
    White,
}

enum CharacrterType {
    Mage,
    Warrior,
    Archer,
    Wizard,
}

struct VehicleTuple(String, String);


struct Vehicle {
    manufactuer: String,
    name: String,
    year: String,
    category: String,
    color: VehicleColor,
}

impl Vehicle {
    fn change_color(&mut   self,newColor:VehicleColor){
       self.color = newColor;
    }
}

#[derive(Debug)]
struct User {
    name: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
