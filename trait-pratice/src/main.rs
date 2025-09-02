use trait_pratice::{SocialPost, NewsArticle,Summary};

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
