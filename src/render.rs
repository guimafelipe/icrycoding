use pulldown_cmark::{Options, Parser};
use tera::{Context, Tera};

#[derive(serde::Serialize)]
pub struct PostData {
    pub title: String,
    pub date: String,
    pub filename: String,
}

pub fn render_main_page(tera: &Tera, posts: &Vec<PostData>) -> String {
    let mut res = String::new();
    for post in posts {
        res.push_str(&post.title);
        res.push('\n')
    }

    let mut context = Context::new();
    context.insert("frontmatters", &posts);

    tera.render("main.html", &context).unwrap()
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
