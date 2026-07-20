use axum::Router;
use axum::routing::{get, post};

#[tokio::main]
async fn main(){
    async fn index()->&'static str{"home"}
    async fn about()->&'static str{"about"}
    async fn hello()->&'static str{"hello world"}
    async fn create_user()->&'static str{"creating new user"}
    async fn list_user()->&'static str{"list of user"}
    let app = Router::new()
        .route("/", get(index))
        .route("/about", get(about))
        .route("/hello", get(hello))
        .route("/user", get(list_user).post(create_user));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Listing in http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
