use lib::notify;
use lib::NewsArticle;
use lib::Summary;
use lib::Summary2;
use lib::Summary3;
use lib::Tweet;

fn main() {
    //let's the necessary modules and types from our library lib.rs
    //(look also Cargo.toml for inclusion)
    //let's build an instance of our struct Tweet and let's use the
    //summarize() method that we have implemented in lib.rs

    let tweet1 = Tweet {
        username: String::from("user1"),
        content: String::from("content1 content1 content1"),
        reply: false,
        retweet: false,
    };

    println!("\ntweet1: {}", tweet1.summarize());

    //let's do the same with out NewsArticle struct

    let article1 = NewsArticle {
        headline: String::from("title"),
        location: String::from("italy"),
        author: String::from("admin"),
        content: String::from("content2 content2 content2"),
    };

    println!("\narticle1: {}", article1.summarize());

    println!("\narticle1 [default]: {}", article1.summarize2());

    println!("\ntweet1 [default with call]: {}", tweet1.summarize3());

    println!("");
    notify(article1);
    notify(tweet1);
}
