use std::fmt::Display;

fn main() {
    println!("{}", largest(&vec!['a', 'b']));
    println!("{}", largest(&vec![2, 1]));


    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    notify(&tweet);

    Pair::new(1, 2).cmp_display();
    // Pair::new(&tweet, &tweet).cmp_display(); // no compila
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

trait Show {
    fn show(&self) -> String;
}

impl<T: Summary> Show for T {
    fn show(&self) -> String {
        self.summarize()
    }
}

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
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("lala"),
        reply: false,
        retweet: false,
    }
}

fn some_function<T, U>(t: &T, u: &U) -> ()
    where
        T: Display + Clone,
        U: Clone + PartialOrd,
{
    todo!()
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify2<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news! {}, {}", item1.summarize(), item2.summarize());
}

pub fn notify3<T: Summary + Display>(item: &T) {
    todo!()
}

trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}, ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

