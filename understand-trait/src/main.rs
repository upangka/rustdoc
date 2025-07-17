use understand_trait::HelloExt;
use understand_trait::NewsArticle;
use understand_trait::Summary;
fn main() {
    let name = String::from("Pkmer");
    name.hello();

    let article = NewsArticle {
        headline: String::from("企鹅队再次夺得斯坦利杯冠军！"),
        location: String::from("中国北京"),
        author: String::from("冰雪"),
        content: String::from(
            "北京企鹅队展现了惊人的实力，\
             再次成为 NHL 冰球联盟中最优秀的球队。",
        ),
    };

    println!("{:#?}", article.summarize());
}
