mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast : &str)=>Breakfast{
            Breakfast{
                toast:String::from(toast),
                seasonal_fruit:String::from("peaches")
            }
        }
    }
}


pub fn eat_at_restuarant(){
    // order a breakfast in the summer with Rye toast

    let mut meal = back_of_house::Breakfast::summer("Rye")

    meal.toast = String::from("wheat")

    println!("I'd like {} toast please", meal.toast)

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}
