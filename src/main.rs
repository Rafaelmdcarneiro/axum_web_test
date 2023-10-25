use std::net::SocketAddr;

use axum::{
    http::StatusCode, 
    routing::{get, post}, 
        Json, Router,
};

use axum_web_test::{create_user::create_user, get_user::get_user, users_db::UsersDb;
use serde_json::{json, Value};

#[tokio::main]

async fn main() {
    let address = SocketAddr::from(([127, 0, 0, 1], 8080));

    let users_db = UsersDb::default();

    let users_api Router::with_state(users_db)
        .route("/", post(create_user);
        .route("/:id", get(get_user));

    let api = Router::new()
        .nest("/users", users_api)
        .fallback(api_fallback);

    let app = Router::new().nest("/api", api);

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();   
}

async fn api_fallback() -> (StatusCode, Json<Value>) {
    let body = json!({
        "status": 404,
        "message": "Not Found"
    });

    (StatusCode::NOT_FOUND, Json(body))
}
