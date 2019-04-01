pub trait Summary {
    fn summarize(&self) -> String {
        String::from("Read more...")
    }
}

impl Summary for Vec<i32> {
    fn summarize(&self) -> String {
        let mut sum: i32 = 0;
        for &elem in self {
            sum += elem;
        }
        sum.to_string()
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {}


fn main() {
    let tweet = Tweet {
        username: String::from("alisalehi"),
        content: String::from("Apparently, the rule:) did not apply to me"),
        reply: false,
        retweet: false,
    };

    println!("tweet summarize: {}", tweet.summarize());
    println!("{}", tweet.content);
}
