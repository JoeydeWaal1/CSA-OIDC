use axum::{routing::{post, get, delete}, Router, extract::State, Json, response::IntoResponse};
use db::InMemDatabase;
use extractor::User;
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;
use axum::http::{Method, HeaderValue, header::AUTHORIZATION, header::CONTENT_TYPE, StatusCode};

mod db;
mod extractor;
mod opa;


#[tokio::main]
async fn main() {
    let appstate = db::InMemDatabase::new();

    opa::init_opa().await;
    // opa::is_student("r15".into()).await;

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, CONTENT_TYPE]);

    let app = Router::new()
        .route("/todos", get(get_todo))
        .route("/todos", post(add_todo))
        .route("/todos", delete(delete_todo))
        .with_state(appstate)
        .layer(cors);

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}


pub async fn get_todo(State(appstate): State<InMemDatabase>, user: User ) -> Json<Vec<String>> {
    let Some(val) = appstate.0.get(&user.name) else {
        return Json(vec![]);
    };
    Json(val.todos.clone())
}

#[derive(Deserialize, Serialize)]
pub struct NewTodo {
    todo: String
}

pub async fn add_todo(
    State(appstate): State<InMemDatabase>,
    user: User,
    Json(todo): Json<NewTodo>,
)
    -> impl IntoResponse
{
    let mut val = appstate.0.entry(user.name).or_insert(db::Row { todos: Vec::new()});
    val.todos.push(todo.todo);
    (StatusCode::OK).into_response()
}


pub async fn delete_todo(
    State(appstate): State<InMemDatabase>,
    user: User,
)
    -> StatusCode
{
    if opa::is_student(user.email).await.unwrap() {
        return StatusCode::UNAUTHORIZED;
    }
    let Some(mut val) = appstate.0.get_mut(&user.name) else {
        return StatusCode::OK;
    };
    let value = val.value_mut();
    value.todos.clear();
    StatusCode::OK
}
