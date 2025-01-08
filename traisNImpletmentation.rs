trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    headline: String,
    content: String,
}

struct BlogPost {
    title: String,
    body: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("NewsArticle: {} - {}", self.headline, self.content)
    }
}

impl Summary for BlogPost {
    fn summarize(&self) -> String {
        format!("BlogPost: {} - {}", self.title, self.body)
    }
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Breaking News"),
        content: String::from("Rust is awesome!"),
    };

    let blog = BlogPost {
        title: String::from("My Rust Journey"),
        body: String::from("Learning Rust is fun and challenging."),
    };

    println!("Article summary: {}", article.summarize());
    println!("Blog summary: {}", blog.summarize());
}
