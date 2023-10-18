use axum::{routing::{get, delete}, Router, response::IntoResponse, extract::{Path, State}, Form, http::StatusCode, };
use askama::Template;
use sqlx::PgPool;
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Serialize, Deserialize)]
struct Todo {
    id: i32,
    description: String, 
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
struct TodoNew {
    description: String, 
}

#[derive(Clone)]
struct AppState {
	db: PgPool,
}

#[shuttle_runtime::main]
async fn main(
	#[shuttle_shared_db::Postgres] db: PgPool,
) -> shuttle_axum::ShuttleAxum {
	sqlx::migrate!().run(&db).await.expect("Looks like something went wrong with migrations :(");

	let state = AppState { db };

    let router = Router::new().route("/", get(homepage)).route("/todos", get(fetch_todos).post(create_todo)).route("/todos/:id", delete(delete_todo)).with_state(state);

    Ok(router.into())
}

async fn homepage() -> impl IntoResponse {
    HelloTemplate 
}

async fn fetch_todos(State(state): State<AppState>) -> impl IntoResponse {
	let todos = sqlx::query_as::<_, Todo>("SELECT * FROM TODOS").fetch_all(&state.db).await.unwrap();

	Records { todos }
}

async fn create_todo(State(state): State<AppState>, Form(form): Form<TodoNew>) -> impl IntoResponse {
	let todo = sqlx::query_as::<_, Todo>("INSERT INTO TODOS (description) VALUES ($1) RETURNING id, description").bind(form.description).fetch_one(&state.db).await.unwrap();
	
	TodoNewTemplate { todo }
}
 
async fn delete_todo(State(state): State<AppState>, Path(id): Path<i32>) -> impl IntoResponse {
	sqlx::query("DELETE FROM TODOS WHERE ID = $1").bind(id).execute(&state.db).await.unwrap();
	
	StatusCode::OK
}
 
#[derive(Template)]
#[template(path = "index.html")]
struct HelloTemplate;

#[derive(Template)]
#[template(path = "meme.html")]
struct Records {
	todos: Vec<Todo>
} 

#[derive(Template)]
#[template(path = "todo.html")]
struct TodoNewTemplate {
	todo: Todo 
} 
