pub trait Summary {
    fn author(&self) -> String;

    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn author(&self) -> String {
        format!("@{}", self.author)
    }
    fn summarize(&self) -> String {
        format!("{} by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}
impl Summary for Tweet {
    fn author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn traits() {
    println!("=========================");
    println!("Using traits in Rust");

    let article = NewsArticle {
        headline: String::from("Rust 1.60 Released"),
        location: String::from("San Francisco, CA"),
        author: String::from("Jane Doe"),
        content: String::from("Rust 1.60 has been released with exciting new features!"),
    };

    println!("New article: {}", article.summarize());

    let tweet = Tweet {
        username: String::from("rustacean"),
        content: String::from("Loving the new features in Rust 1.60!"),
        reply: false,
        repost: false,
    };

    println!("Tweet by {}", tweet.summarize());

    notify(&tweet);
    notify_with_bound_example(&article);

    // Example of a trait with a lifetime parameter
    trait DisplayWithLifetime<'a> {
        fn display(&self) -> &'a str;
    }

    struct Example<'a> {
        value: &'a str,
    }

    impl<'a> DisplayWithLifetime<'a> for Example<'a> {
        fn display(&self) -> &'a str {
            self.value
        }
    }

    let example = Example {
        value: "Hello, Rust!",
    };
    println!("{}", example.display());
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

//using trait bounds
fn notify_with_bound_example<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
