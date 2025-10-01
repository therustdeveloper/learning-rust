//! src/traits/media_aggregator.rs
pub fn index(show: bool) {
    if show {
        println!("--- Media Aggregator ---");

        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };
        // we can call summarize on instances
        println!("1 new tweet: {}", tweet.summarize());

        let article = NewsArticle {
            headline: String::from("Learning Rust!"),
            location: String::from("Envigado"),
            author: String::from("Will"),
            content: String::from("Learning Rust is fantastic!"),
        };
        println!("1 new article: {}", article.summarize());
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more form {}...", self.summarize_author())
    }
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!(
            "{}, by {} ({}) and \n{}",
            self.headline, self.author, self.location, self.content
        )
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!(
            "{}: {}, reply: {}, retweet: {}",
            self.username, self.content, self.reply, self.retweet
        )
    }
}
