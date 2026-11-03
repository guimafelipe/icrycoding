use axum::{Router, extract::Path, response::Html, routing::get};
use std::sync::Arc;
use tera::Tera;
use tower_http::services::ServeDir;

mod posts;
mod render;
use crate::{posts::*, render::*};

async fn about() {
    println!("Fui chamado")
}

fn get_post(tera: Arc<Tera>, Path(title): Path<String>) -> Html<String> {
    let (content, metadata) = posts::get_post(&title[..]).unwrap();
    let post_data = PostData {
        filename: title,
        title: metadata.title,
        date: metadata.date.to_string(),
    };

    Html(render_post_page(&tera, &content, &post_data))
}

fn get_main_page(tera: Arc<Tera>) -> Html<String> {
    let posts_frontmatter = get_all_frontmatter();
    let mut posts: Vec<PostData> = vec![];

    for frontmatter in posts_frontmatter {
        posts.push(PostData {
            title: frontmatter.title,
            date: frontmatter.date.to_string(),
            filename: frontmatter.filename,
        });
    }

    Html(render_main_page(&tera, &posts))
}

#[tokio::main]
async fn main() {
    let mut tera = Tera::default();
    tera.load_from_glob("templates/**/*.html").unwrap();

    let teraref = Arc::new(tera);

    let app = Router::new()
        .route(
            "/",
            get({
                let ctera = Arc::clone(&teraref);
                || async { get_main_page(ctera) }
            }),
        )
        .route("/about", get(about))
        .route(
            "/posts/{title}",
            get({
                let ctera = Arc::clone(&teraref);
                |title| async { get_post(ctera, title) }
            }),
        )
        .nest_service("/assets", ServeDir::new("assets"))
        .nest_service("/styles", ServeDir::new("styles"));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
