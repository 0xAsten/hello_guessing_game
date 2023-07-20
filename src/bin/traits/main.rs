use std::fmt::{Debug, Display};

struct NewsArticle {
    author: String,
    headline: String,
    content: String,
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.content, self.author)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.username, self.content)
    }
}

trait Summary {
    fn summarize(&self) -> String;
}

// trait as parameter
fn notify_1(item: &impl Summary) {
    println!("notify: {}", item.summarize());
}

fn notify_2<T: Summary>(item: &T) {
    println!("notify: {}", item.summarize());
}

fn notify_3(item1: &impl Summary, item2: &impl Summary) {
    // ...
}

fn notify_4<T: Summary>(item1: &T, item2: &T) {
    // ...
}

// where clause
fn some_fun_1<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) {
    // ...
}

fn some_fun_2<T, U>(t: &T, u: &U)
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // ...
}

// Return type that implemts some trait
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: "0x".to_string(),
        content: "yes, wonderful world".to_string(),
        reply: false,
    }
}

// Using trait bounds(generics with traits) to conditionally implemet methods
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn compare(&self) {
        if self.x > self.y {
            // ...
        } else {
            // ...
        }
    }
}

fn main() {
    let article = NewsArticle {
        author: "0x".to_string(),
        headline: "hello world".to_string(),
        content: "yes, wonderful world".to_string(),
    };

    println!("{}", article.summarize());

    notify_1(&article);
}
