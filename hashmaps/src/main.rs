use std::collections::HashMap;


fn main (){
   let mut book_reviews = HashMap::new();
   book_reviews.insert("Adventure of huckleberry", "Ofuzor Chukwuemeke");
   book_reviews.insert("The 5:00am club", "Robin Sharma");
   book_reviews.insert("Atomic Habits", "James Clear");
   book_reviews.insert("Rework", "DHH , JASON FRIED");

   if !book_reviews.contains_key("Adventure of huckleberry"){
     println!("We've got {} reviews , but Les Miserables aint one.",book_reviews.len());
   }

   book_reviews.remove("Adventure of huckleberry");

   let to_find = ["Pride and Prejudice","Alice's Adventure in Wonderland"];

   for &book in &to_find{
      match book_reviews.get(book){
        Some(review) =>println!("{book}:{review}"),
        None =>println!("{book} is unreviewed")
      }
   }

   println!("Review for jane:{}",book_reviews["pride is a devil"]);
}