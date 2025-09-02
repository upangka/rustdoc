
mod trait_bound_conditionally;

use std::fmt::{Debug, Display};
use trait_pratice::{NewsArticle, SocialPost, Summary};

fn main() {
    let social_post = SocialPost {
        username: String::from("Pkmer"),
        content: String::from("Rust需要语言设计很独具一帜"),
        reply: true,
        repost: false,
    };

    println!("{}", social_post.summarize());

    let news_article = NewsArticle {
        headline: String::from("Rust 语言"),
        location: String::from("中国"),
        author: String::from("Pkmer"),
        content: String::from("Rust 语言需要语言设计很独具一帜"),
    };

    println!("{}", news_article.summarize());
}

fn notify(item: &(impl Display + Summary)) {
    println!("重磅消息 {}", item.summarize())
}

fn some_function_v1<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    0
}
fn some_function_v2<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0
}
