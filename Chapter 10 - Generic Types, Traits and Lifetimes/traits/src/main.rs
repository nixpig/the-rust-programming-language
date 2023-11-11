trait Summary {
    fn summarise(&self) -> String;
}

fn notify(item: &impl Summary) {
    println!("New item: {}", item.summarise())
}

trait ReadMore {
    fn author_details(&self) -> String;

    // default implentation
    fn read(&self) -> String {
        format!("Read more from: {}", self.author_details())
    }
}

struct NewsArticle {
    headline: String,
    author: String,
}

impl Summary for NewsArticle {
    fn summarise(&self) -> String {
        format!("{} by {}", self.headline, self.author)
    }
}

// use default implemenation of trait
impl ReadMore for NewsArticle {
    fn author_details(&self) -> String {
        format!("{}", self.author)
    }
}

struct Tweet {
    author: String,
    tweet: String,
}

impl Summary for Tweet {
    fn summarise(&self) -> String {
        format!("{} (@{})", self.tweet, self.author)
    }
}

impl ReadMore for Tweet {
    fn read(&self) -> String {
        String::from("Follow.")
    }

    fn author_details(&self) -> String {
        format!("@{}", self.author)
    }
}

fn summarisable() -> impl Summary {
    Tweet {
        author: String::from("fasdf"),
        tweet: String::from("some tweet that's summarisable"),
    }
}

fn main() {
    let tweet = Tweet {
        author: String::from("nixpig"),
        tweet: String::from("Having fun learning Rust 🦀"),
    };

    let news = NewsArticle {
        author: String::from("Anon"),
        headline: String::from("More drama from the Rust Foundation!"),
    };

    println!("{}", tweet.summarise());
    println!("{}", tweet.read());
    println!("{}", news.summarise());
    println!("{}", news.read());

    notify(&tweet);
    notify(&news);
}
