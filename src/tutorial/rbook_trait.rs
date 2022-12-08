pub trait Summerizable {
    fn author_summary(&self) -> String;

    fn summary(&self) -> String {
        format!("(Read more from {}...)", self.author_summary())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}

impl Summerizable for NewsArticle {
    fn author_summary(&self) -> String {
        format!("@{}", self.author)
    }
}

pub fn notify<T: Summerizable>(item: T) {
    println!("Breaking news! {}", item.summary());
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summerizable for Tweet {
    fn author_summary(&self) -> String {
        format!("@{}", self.username)
    }
}


pub fn example() {

    let tweet = Tweet {
        username: String::from("hosrse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false
    };

    println!("1 new tweet: {}", tweet.summary());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best
        hockey team in the NHL."),
    };
    
    println!("New article available! {}", article.summary());

}