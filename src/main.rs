use axum::{Router, extract::Path, routing::get};

mod posts;
use crate::posts::*;

async fn about() {
    println!("Fui chamado")
}

async fn get_post(Path(title): Path<String>) -> String {
    /*let posts_path = format!("content/posts/{title}.md");
    let res = fs::read_to_string(posts_path).unwrap_or("Not found".to_string());
    println!("{}", res);
    res
    */

    let (content, _) = posts::get_post(&title[..]).unwrap();
    content
}

async fn get_main_page() -> String {
    String::from("Hello, world!")
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(get_main_page))
        .route("/about", get(about))
        .route("/posts/{title}", get(get_post));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
