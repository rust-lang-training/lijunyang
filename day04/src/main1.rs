fn main() {
    println!("Hello, world!");
    let a = 1;

    let newArticle = NewsArticle {
        headline: "a".to_string(),
        location: "b".to_string(),
        author: "c".to_string(),
        content: "c".to_string(),
    };
    let tweet = Tweet {
        username: "a".to_string(),
        content: "a".to_string(),
        reply: true,
        retweet: false,
    };

    notify(&tweet);
    notify(&newArticle);
    notify2(&tweet);
    notify2(&newArticle);
    notify3(&tweet);
    notify3(&newArticle);
}
// trait 可分为三类
// 语言扩展trait 标记性trait 普通trait
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: "a".to_string(),
        content: "a".to_string(),
        reply: true,
        retweet: false,
    }
}

fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
fn notify3<T>(item: &T)
where
    T: Summary,
{
    println!("Breaking news! {}", item.summarize());
}
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!(
            "{} by {} ({})",
            &self.headline, &self.author, &self.location
        )
    }
}
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", &self.username, &self.content)
    }
}
// 定义 trait
trait Summary {
    fn summarize(&self) -> String;
}
