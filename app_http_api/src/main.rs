use std::net::SocketAddr;

use serde::{Serialize, Deserialize};

use axum::{
    routing::{get,post},
    extract::Query,
    http::StatusCode,
    response::{IntoResponse},
    Json,Router
};

#[tokio::main]
async fn main() {
    let app = Router::new()
      .route("/greet", get(greet))
      .route("/users", post(create_user));
    let addr = SocketAddr::from(([0,0,0,0],8888));
    axum::Server::bind(&addr)
      .serve(app.into_make_service())
      .await
      .unwrap()
}



async fn greet(Query(params): Query<GreetParams>) -> String {
    format!("Hello! {:?}",params.name)
}

#[derive(Deserialize)]
struct GreetParams {
    name: String
}

async fn create_user(
    Json(payload): Json<CreateUser>
) -> impl IntoResponse {
    let u = User {
        id: String::from("xxxxxxxx"),
        username: payload.username
    };
    (StatusCode::CREATED,Json(u))
}

#[derive(Deserialize)]
struct CreateUser {
    username: String
}

#[derive(Serialize)]
struct User {
    id: String,
    username: String
}