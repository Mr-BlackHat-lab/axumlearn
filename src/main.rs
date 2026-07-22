use axum::routing::post;
use axum::{
    response::{IntoResponse, Response},
    extract::{Path,Query},
    routing::get,
    Router,
    Json,
    http::StatusCode,
    response::Html,
    Form,
};
use serde::{Serialize, Deserialize};
use std::sync::Arc;

#[derive(Deserialize)]
struct Pagination{
    page:Option<u32>,
    per_page:Option<u32>
}
#[derive(Deserialize)]
struct LoginForm{
    username:String,
    password:String,
}
#[derive(Serialize, Deserialize)]
struct User{
    id:u64,
    name:String,
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



    async fn list_items(Query(pagination): Query<Pagination>)->String{
        let page = pagination.page.unwrap_or(1);
        let per_page = pagination.per_page.unwrap_or(20);
        format!("Page {page}, {per_page} items")
    }

    async fn login(Form(input): Form<LoginForm>)->String{
       format!("login attempt: {}", input.username)
    }
    async fn create_user(Json(input): Json<User>)->(StatusCode,String){
        (
            StatusCode::CREATED,
            format!("Created user: {} ,id:({})", input.name, input.id)
        )
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

    async fn multiinput(
        Path(id): Path<u64>,                   // from URL
        Query(params): Query<Pagination>,      // from query string
        Json(body): Json<User>,          // from body (must be last)
    ) -> String {
        let page = params.page.unwrap_or(1);
        let per_page = params.per_page.unwrap_or(20);
        let name = body.name;
        let idd = body.id;
        format!("id:{id} Page:{page}, {per_page}:items || Body name:{name} id:{idd}")
    }

    let tyeps_of_return = Router::new()
        .route("/plain", get(plain))
        .route("/htmltype", get(htmltype))
        .route("/status", get(no_content))
        .route("/jsontype", get(jsontype))
        .route("/mixtuple", get(mixtuple));
    let user = Router::new()
        .route("/", get(list_user))
        .route("/login", post(login))
        .route("/create_user", post(create_user))
        .route("/{id}", get(list_single_user))
        .route("/name/{username}/id/{id}", get(list_user_by_name));
    let base = Router::new()
        .route("/", get(index))
        .route("/about", get(about))
        .route("/hello", get(hello));
    let files = Router::new()
        .route("/{*path}", get(serve_file));
    let item = Router::new()
        .route("/list_items", get(list_items))
        .route("/multinput/{id}", post(multiinput));
    let basic_routes = Router::new()
        .merge(base)
        .nest("/files", files)
        .nest("/user", user)
        .nest("/type", tyeps_of_return)
        .nest("/items", item);

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
