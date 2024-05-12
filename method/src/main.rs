#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32,
}


// impl Rectangle {
//     fn area(&self)->{
//       self.width * self.height
//     }
// }


// impl Rectangle{
//     fn width(){

//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     if rect1.width() {
//         println!("The rectangle has a nonzero width; it is {}", rect1.width);
//     }
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }


// Multiple impl Blocks
// Each struct is allowed to have multiple impl blocks. For example, Listing 5-15 is equivalent to the code shown in Listing 5-16, which has each method in its own impl block.

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main(){
    let rect1 = Rectangle{
        width:30,
        height:50,
    };
    println!("The area of the rectangle is {} square pixels",rect1.area())
}