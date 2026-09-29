use axum::{Router, extract::Path, response::Html, routing::get};
use std::fs;
use tera::Tera;
use tower_http::services::ServeDir;

mod posts;
mod render;
use crate::{posts::*, render::*};

async fn about() {
    println!("Fui chamado")
}

async fn get_post(Path(title): Path<String>) -> String {
    let (content, _) = posts::get_post(&title[..]).unwrap();
    content
}

async fn get_main_page() -> Html<String> {
    let posts_frontmatter = get_all_frontmatter();
    let mut posts: Vec<PostData> = vec![];

    for frontmatter in posts_frontmatter {
        posts.push(PostData {
            title: frontmatter.title,
            date: frontmatter.date,
        });
    }

    Html(render_main_page(posts))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(get_main_page))
        .route("/about", get(about))
        .route("/posts/{title}", get(get_post))
        .nest_service("/assets", ServeDir::new("assets"))
        .nest_service("/styles", ServeDir::new("styles"));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
