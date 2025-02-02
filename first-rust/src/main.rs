use std::{
    fmt::{Debug, Display},
    iter::Sum,
};

fn main() {
    let tweet = Tweet {
        username: String::from("Test User"),
        content: String::from("Test 1234566ld;fksdfdlfdsdfds"),
        reply: false,
        retweet: false,
    };
    println!("{}", tweet.summarize());
}

trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more...) {}", self.summarize_author())
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

fn notify<T: Summary>(item: &T, item2: &T) {
    println!("Breaking news! {} {}", item.summarize(), item2.summarize())
}

fn some_func<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
}

fn returns_summarize(switch: bool) -> Box<dyn Summary> {
    if switch {
        Box::new(NewsArticle {
            headline: String::from("test"),
            location: String::from("TH"),
            author: String::from("A"),
            content: String::from("..."),
        })
    } else {
        Box::new(Tweet {
            username: String::from("Test User"),
            content: String::from("Test 1234566ld;fksdfdlfdsdfds"),
            reply: false,
            retweet: false,
        })
    }
}
