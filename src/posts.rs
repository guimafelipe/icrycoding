use chrono::*;
use frontmatter_gen::extract;
use std::fs;

pub struct PostMetadata {
    pub title: String,
    pub date: NaiveDate,
}

pub fn get_all_frontmatter() -> Vec<PostMetadata> {
    let content = fs::read_to_string("content/posts/test1/post.md").unwrap();
    let result = extract(&content);
    assert!(result.is_ok());

    let mut res: Vec<PostMetadata> = Vec::new();

    let (frontmatter, _) = result.unwrap();

    let test = frontmatter.get("date").unwrap();

    let title = frontmatter.get("title").unwrap();
    let date_str = frontmatter.get("date").unwrap();

    let metadata = PostMetadata {
        title: String::from(title.as_str().unwrap()),
        date: NaiveDate::parse_from_str(date_str.as_str().unwrap(), "%Y-%m-%d").unwrap(),
    };

    res.push(metadata);

    println!("{test}");

    res
}
