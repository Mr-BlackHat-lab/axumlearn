use axum::{
    response::{IntoResponse, Response},
    extract::Path,
    routing::get,
    Router,
    Json,
    http::StatusCode,
    response::Html,
};
use serde::Serialize;

#[derive(Serialize)]
struct User{
    id:u64,
    name:String
}
enum ApiResponse {
    Ok,
    Created,
    JsonData(Vec<User>),
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response{
    match self {
        Self::Ok => StatusCode::OK.into_response(),
        Self::Created=> StatusCode::CREATED.into_response(),
        Self::JsonData(data)=> (StatusCode::OK, Json(data)).into_response(),
    }

}
}
#[tokio::main]
async fn main() {
    async fn index() -> &'static str {
        "home"
    }

    async fn about() -> &'static str {
        "about"
    }

    async fn hello() -> &'static str {
        "hello world"
    }

    async fn create_user() -> &'static str {
        "creating new user"
    }

    async  fn list_user() -> ApiResponse{
        ApiResponse::JsonData(vec![
            User{id:1,name:"Someone".into()},
        ])
    }
    async fn list_single_user(Path(id): Path<String>) -> String {//extracting id form http link requset
        println!("id: {}", id);
        format!("single user id {}", id)
    }
    // Multiple path params
    async fn list_user_by_name(Path((username, id)): Path<(String, u64)>) -> ApiResponse {
        ApiResponse::JsonData(vec![
            User{id,name:username.into()}
        ])
    }
    async fn serve_file(Path(path): Path<String>) -> String {
        println!("Requested file: {}", path);
        format!("Requested file: {}", path)
    }
    async fn plain()-> &'static str{
        "plain"
    }
    async fn htmltype()->Html<&'static str>{
        Html("<h1>html page</h1>")
    }
    async fn no_content()->StatusCode{
        StatusCode::NO_CONTENT
    }
    async fn jsontype()->Json<serde_json::Value>{
        Json(serde_json::json!({"message":"json type return"}))
    }
    async fn mixtuple()->(StatusCode,Json<serde_json::Value>){
        (StatusCode::OK, Json(serde_json::json!({"message":"multiple tpye return in tuple"})))
    }


    let tyeps_of_return = Router::new()
        .route("/plain", get(plain))
        .route("/htmltype", get(htmltype))
        .route("/status", get(no_content))
        .route("/jsontype", get(jsontype))
        .route("/mixtuple", get(mixtuple));
    let basic_routes = Router::new()
        .route("/", get(index))
        .route("/about", get(about))
        .route("/hello", get(hello))
        .route("/user", get(list_user).post(create_user))
        .route("/user/{id}", get(list_single_user))
        .route("/user/name/{name}/id/{id}", get(list_user_by_name))
        .route("/files/{*path}", get(serve_file))
        .nest("/type", tyeps_of_return);

    let app = Router::new()
        .nest("/v1/api", basic_routes);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Listening on http://127.0.0.1:3000/v1/api");

    axum::serve(listener, app)
        .await
        .unwrap();
}
