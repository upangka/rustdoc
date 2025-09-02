pub trait Summary {
    fn summarize(&self) -> String {
        format!("(更多) 来自 {}", self.summarize_author())
    }

    fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// 实现trait
// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{} by {} {}", self.headline, self.author, self.location)
//     }
// }

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
         // self.author // ❌，这里没有所有权的转移
        format!("{}", self.author)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

// 实现trait
impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{} by {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("{}", self.username)
    }
}
