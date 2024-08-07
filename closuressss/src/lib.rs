#[derive(PartialEq,Debug)]
struct Shoe{
    size:u32,
    style:String
}

fn shoes_in_size(shoe:Vec<Shoe>,shoe_size:u32)->Vec<Shoe>{
    shoe.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn filter_by_size(){
        let shoes = vec![
            Shoe{
                size:10,
                style:String::from("sneaker")
            },
            Shoe{
                size:13,
                style:String::from("Nike Dunks")
            },
            Shoe{
                size:10,
                style:String::from("Sandals")
            },
        ];
        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}