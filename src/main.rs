use axum::{Router, routing::get};

#[tokio::main]
async fn main(){
    let app = Router::new().route("/hello", get(|| async {"hello world"}));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Listing in http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
