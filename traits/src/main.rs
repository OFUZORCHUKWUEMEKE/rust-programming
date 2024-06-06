use aggregator::{Summmary,Tweet};

fn main(){
    let tweet = Tweet{
        username:String::from("horse_ebook"),
        content:String::fron("Of course , as you probaby know , people"),
        reply:false,
        retweet:false
    }
    println!("1 new tweet: {}", tweet.summarize());
}