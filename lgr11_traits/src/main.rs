use std::fmt::{Debug, Display};

fn main() {
    tweet_and_article();
}

/* 11.0 Traits */
pub struct NewArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewArticle {
    // fn summarize(&self) -> String {
    //     format!("{}, by {}", self.headline, self.author)
    // }

    //there's no default implementation in trait so we have to implement it
    fn summarize_author(&self) -> String {
        format!("{} ", self.author)
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
        format!("{}: {}", self.username, self.content)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        //if defined then default
        String::from("Read more...")
    }
}

fn tweet_and_article() {
    let tweet = Tweet {
        username: String::from("@johndoe"),
        content: String::from("Hello World!"),
        reply: false,
        retweet: false,
    };

    let article = NewArticle {
        author: String::from("John Doe"),
        headline: String::from("The Sky is Falling"),
        content: String::from("The sky is not actually falling"),
    };

    println!("Tweet summary: {}", tweet.summarize());
    println!("Article summary: {}", article.summarize());

    notify(&article);
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
//alternative
pub fn notifyA<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

/* 11.1 variants of defining generic in functions */
//they have the same impl but could be different type
pub fn notifyB(item1: &impl Summary, item2: &impl Summary) {
    //..
}
//they are exact the same type
pub fn notifyBB<T: Summary>(item1: &T, item2: &T) {
    //..
}
//contrary
//specified for one argument
pub fn notifyC(item1: &(impl Summary + Display), item2: &impl Summary) {
    //..
}
//has to display both traits
pub fn notifyCC<T: Summary + Display>(item1: &T, item2: &T) {
    //..
}

/* 11.2 readability */
fn some_functionA<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) {
    //...
}
//alternative
fn some_functionB<T, U>(t: &T, u: &U)
where
    T: Display + Clone,
    U: Clone + Debug,
{
    //...
}

/* 11.3 Trait bounds */
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        //Self indicated on struct or impl (=> Pair)
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
