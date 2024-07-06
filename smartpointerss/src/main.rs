struct NameSpace {
    date: String,
    name: String,
    phonenumber: u8,
}

impl NameSpace {
    fn new(&self, name: String)->NameSpace{
        NameSpace {
            date: name,
            name:name,
            phonenumber: 290,
        }
    }
}

fn main() {
    let b = Box::new(5);
    println!("b ={}", b);
}
