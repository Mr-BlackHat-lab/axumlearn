use axum::{Router, routing::get};

#[tokio::main]
async fn main(){
    async fn index()->&'static str{"home"}
    async fn about()->&'static str{"about"}
    async fn hello()->&'static str{"hello world"}
    let app = Router::new()
        .route("/", get(index))
        .route("/about", get(about))
        .route("/hello", get(hello));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Listing in http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
