use axum::{Router, extract::Path, response::Html, routing::get};
use tera::Tera;
use tower_http::services::ServeDir;

mod posts;
mod render;
use crate::{posts::*, render::*};

async fn about() {
    println!("Fui chamado")
}

async fn get_post(Path(title): Path<String>) -> Html<String> {
    let mut tera = Tera::default();
    tera.load_from_glob("templates/**/*.html").unwrap();

    let (content, metadata) = posts::get_post(&title[..]).unwrap();
    let post_data = PostData {
        filename: title,
        title: metadata.title,
        date: metadata.date.to_string(),
    };

    Html(render_post_page(&tera, &content, &post_data))
}

async fn get_main_page() -> Html<String> {
    let mut tera = Tera::default();
    tera.load_from_glob("templates/**/*.html").unwrap();

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
