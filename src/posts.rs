use chrono::*;
use frontmatter_gen::{Frontmatter, extract};
use std::fs;

pub struct PostMetadata {
    pub title: String,
    pub date: NaiveDate,
}

fn convert_to_metadata(frontmatter: Frontmatter) -> PostMetadata {
    let test = frontmatter.get("date").unwrap();

    let title = frontmatter.get("title").unwrap();
    let date_str = frontmatter.get("date").unwrap();

    PostMetadata {
        title: String::from(title.as_str().unwrap()),
        date: NaiveDate::parse_from_str(date_str.as_str().unwrap(), "%Y-%m-%d").unwrap(),
    }
}

pub fn get_post(name: &str) -> Option<(String, PostMetadata)> {
    let file = fs::read_to_string(format!("content/posts/{name}.md")).unwrap();

    let (frontmatter, content) = extract(&file).unwrap();

    Some((
        String::from(content.trim()),
        convert_to_metadata(frontmatter),
    ))
}

pub fn get_all_frontmatter() -> Vec<PostMetadata> {
    let paths = fs::read_dir("content/posts/").unwrap();
    let mut res: Vec<PostMetadata> = Vec::new();

    for path in paths {
        let file_path = path.unwrap().path();
        println!("Name: {}", file_path.display());

        let content = fs::read_to_string(file_path).unwrap();
        let result = extract(&content);
        assert!(result.is_ok());

        let (frontmatter, _) = result.unwrap();

        res.push(convert_to_metadata(frontmatter));
    }

    res
}

