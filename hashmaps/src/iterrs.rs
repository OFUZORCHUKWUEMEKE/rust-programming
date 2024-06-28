pub fn Iterrs() {
    let  fruit = vec!["Juice", "Food", "Strawberry", "Lime"];

    let mut  fruit_iter = fruit.iter();

    fruit_iter.next();

    for fruit in fruit_iter {
        println!("{}", fruit)
    }
}
