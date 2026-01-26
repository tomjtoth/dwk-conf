use std::sync::LazyLock;
use std::{env, sync::Arc};

use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::put;
use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use sqlx::prelude::FromRow;
use sqlx::{Pool, Postgres, query, query_as};

struct AppState(Pool<Postgres>);

#[derive(Serialize, Deserialize, Clone, FromRow)]
struct Todo {
    id: i32,
    done: bool,
    task: String,
}

static DATABASE_URL: LazyLock<String> =
    LazyLock::new(|| env::var("DATABASE_URL").expect("missing env var DATABASE_URL"));

#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&*DATABASE_URL)
        .await
        .expect(&format!("unable to connect to DB @ {}", &*DATABASE_URL));

    let state = Arc::new(AppState(pool));

    let app = Router::new()
        .route("/healthz", get(healthz))
        .route("/todos", get(retrieve_todos))
        .route("/todos", post(add_todo))
        .route("/todos/{id}", put(mark_done))
        .with_state(state);

    let ip = env::var("IP").expect("missing env var IP");
    let port = env::var("PORT").expect("missing env var PORT");

    let addr = format!("{}:{}", &ip, &port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("listening at http://{}/todos", &addr);
    axum::serve(listener, app).await.unwrap();
}

async fn retrieve_todos(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let todos: Vec<Todo> = query_as("SELECT * FROM todos")
        .fetch_all(&state.0)
        .await
        .expect("retrieving todos failed");

    println!("serving {} todos", todos.len());

    axum::response::Json::from(todos)
}

async fn add_todo(
    State(state): State<Arc<AppState>>,
    Json(todo): Json<String>,
) -> Result<axum::Json<Todo>, StatusCode> {
    let len = todo.len();

    println!("Adding todo:\n\t{}", todo);

    if len > 140 {
        eprintln!("todo longer, than 140 ({}), aborting!", len);
        return Err(StatusCode::BAD_REQUEST);
    }

    let todo: Todo = query_as("INSERT INTO todos (task) VALUES ($1) RETURNING *")
        .bind(todo)
        .fetch_one(&state.0)
        .await
        .expect("adding todo failed");

    Ok(axum::response::Json::from(todo))
}

async fn mark_done(State(state): State<Arc<AppState>>, Path(id): Path<i64>) -> impl IntoResponse {
    query("UPDATE todos SET done = true WHERE id = $1")
        .bind(id)
        .execute(&state.0)
        .await
        .expect("marking todo done failed");

    StatusCode::OK
}

async fn healthz(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let res = query("SELECT 1").execute(&state.0).await;

    if res.is_ok() {
        StatusCode::OK
    } else {
        StatusCode::FAILED_DEPENDENCY
    }
}
