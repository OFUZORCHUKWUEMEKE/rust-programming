pub struct NewsArticle {
    pub headline:String,
    pub location: String,
    pub author :String
    pub content:String
}


impl Summmary for NewsArticle {
    fn summarize(&self)->String{
        format!("{} , by {} ,({})",self.headline,self.author,self.location)
    }
}

pub struct Tweet{
    pub username:String,
    pub content:String,
    pub reply :bool,
    pub retweet:bool
}

impl Summmary for Tweet{
    fn summarize(&self)->String{
        format!("{}:{}",self.username,self.content)
    }
}
