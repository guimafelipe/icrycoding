use chrono::*;
use frontmatter_gen::{Frontmatter, extract};
use std::fs;
use std::path::Path;

pub struct PostMetadata {
    pub title: String,
    pub date: NaiveDate,
    pub filename: String,
}

fn convert_to_metadata(frontmatter: Frontmatter, filename: String) -> PostMetadata {
    let title = frontmatter.get("title").unwrap();
    let date_str = frontmatter.get("date").unwrap();

    PostMetadata {
        title: String::from(title.as_str().unwrap()),
        date: NaiveDate::parse_from_str(date_str.as_str().unwrap(), "%Y-%m-%d").unwrap(),
        filename: filename,
    }
}

pub fn get_post(name: &str) -> Option<(String, PostMetadata)> {
    let file = fs::read_to_string(format!("content/posts/{name}.md")).unwrap();

    let (frontmatter, content) = extract(&file).unwrap();

    Some((
        String::from(content.trim()),
        convert_to_metadata(frontmatter, name.to_string()),
    ))
}

pub fn get_all_frontmatter() -> Vec<PostMetadata> {
    let paths = fs::read_dir("content/posts/").unwrap();
    let mut res: Vec<PostMetadata> = Vec::new();

    for path in paths {
        let path2 = path.unwrap();
        let file_path = path2.path();
        println!("Name: {}", file_path.display());

        let content = fs::read_to_string(file_path).unwrap();
        let result = extract(&content);
        assert!(result.is_ok());

        let (frontmatter, _) = result.unwrap();

        let file_name = path2.file_name();
        let mut filename_str = file_name.into_string().unwrap();

        for _ in 0..".md".len() {
            filename_str.pop();
        }

        println!("{filename_str}");

        res.push(convert_to_metadata(frontmatter, filename_str));
    }

    res
}
