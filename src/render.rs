use chrono::*;
use tera::{Context, Tera};

pub struct PostData {
    pub title: String,
    pub date: NaiveDate,
}

pub fn render_main_page(posts: &Vec<PostData>) -> String {
    let mut res = String::new();
    for post in posts {
        res.push_str(&post.title);
        res.push('\n')
    }
    res
}

pub fn render_post_page(tera: &Tera, content: &String, post_data: &PostData) -> String {
    let mut context = Context::new();
    context.insert("content", &content);
    context.insert("title", &post_data.title);
    context.insert("date", &post_data.date.to_string());

    tera.render("post.html", &context).unwrap()
}
