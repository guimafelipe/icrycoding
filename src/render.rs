use chrono::*;
use pulldown_cmark::{Options, Parser};
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

    let options = Options::empty();

    let parser = Parser::new_ext(content, options);

    let mut html_content = String::new();

    pulldown_cmark::html::push_html(&mut html_content, parser);

    context.insert("content", &html_content);
    context.insert("title", &post_data.title);
    context.insert("date", &post_data.date.to_string());

    tera.render("post.html", &context).unwrap()
}
