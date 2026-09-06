use chrono::*;
use tera::Tera;

pub struct PostData {
    pub title: String,
    pub date: NaiveDate,
}

pub fn render_main_page(posts: Vec<PostData>) -> String {
    let mut res = String::new();
    for post in posts {
        res.push_str(&post.title);
        res.push('\n')
    }
    res
}
