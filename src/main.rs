use axum::{Router, extract::Path, routing::get};
use std::fs;

async fn about() {
    println!("Fui chamado")
}

async fn get_post(Path(title): Path<String>) -> String {
    println!("chamou {title}");
    let posts_path = format!("content/posts/{title}/post.md");
    let res = fs::read_to_string(posts_path).unwrap_or("Not found".to_string());
    println!("{}", res);
    res
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/about", get(about))
        .route("/posts/{title}", get(get_post));

    let test = fs::read_to_string("content/posts/test1/post.md").unwrap();
    println!("{test}");

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
