struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

#[derive(Debug)]
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

struct Thread {
    username: String,
    content: String,
}

// Shared Behaviour of NewsArticle & Trait --> similar to Java Interfaces
trait Summary {
    fn get_author(&self) -> &str; // no body --> has to be defined explicitly in each derived implementation

    fn summarize(&self) -> String {
        format!("{}: (Read More...)", self.get_author()) // default implementation
    }
}

trait MyTrait {
    fn demo(&self);
}

impl Summary for Tweet {
    // similar to @Override in Java or abstract classes & interfaces
    fn summarize(&self) -> String {
        format!("Tweet by {}: {}", self.username, self.content) // returns a formatted string
    }

    fn get_author(&self) -> &str {
        self.username.as_str()
    }
}

impl MyTrait for Tweet {
    fn demo(&self) {
        println!("{:?}", self);
    }
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("News Article by {}: {}", self.author, self.content)
    }

    fn get_author(&self) -> &str {
        self.author.as_str()
    }
}

impl Summary for Thread {
    fn get_author(&self) -> &str {
        self.username.as_str()
    }

    // here we accept the default implementation of summarize() fn
}

fn main() {
    let tweet: Tweet = Tweet {
        username: String::from("Tamojit"),
        content: String::from("Kafka, Jenkins, Microservices, Nest"),
        reply: false,
        retweet: false,
    };
    news_aggregator(&tweet);

    let article: NewsArticle = NewsArticle {
        headline: String::from("RUST"),
        location: String::from("Kolkata"),
        author: String::from("Tamojit"),
        content: String::from("Rust Basics"),
    };
    news_aggregator(&article);

    let thread: Thread = Thread {
        username: String::from("Test"),
        content: String::from("LLD, HLD"),
    };
    news_aggregator(&thread);

    get_news(&thread);
    get_news(&tweet);
    get_news(&article);

    mixup(&tweet, &article);

    display(&tweet); // possible since Tweet implements both Summary & MyTrait
    // display(&article); // NOT possible since NewsArticle doesn't implement MyTrait
}

// fn utilizing/implementing trait behaviour
fn news_aggregator(source: &impl Summary) {
    println!("New News:");
    println!("{}", source.summarize());
}

fn get_news<T: Summary>(source: &T) {
    // T --> generic trait that implements summary
    println!("{}", source.summarize());
}

fn mixup(primary: &impl Summary, other: &impl Summary) { // Tweet, Thread, NewsArticle types can be mixed up now, since they all implement Summary trait
    println!("{} and {}", primary.summarize(), other.summarize());
}

fn display(item: &(impl Summary + MyTrait)) {
    item.demo();
}