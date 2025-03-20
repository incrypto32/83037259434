use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Clone)]
struct Movie {
    id: String,
    name: String,
    year: u16,
    was_good: bool,
}

#[tokio::main]
async fn main() {
    let db = Arc::new(Mutex::new(HashMap::new()));

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/movie/{id}", get(get_movie))
        // `POST /users` goes to `create_user`
        .route("/movie", post(add_movie))
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Starting server");
    axum::serve(listener, app).await.unwrap();
}

async fn get_movie(
    State(db): State<Arc<Mutex<HashMap<String, Movie>>>>,
    Path(id): Path<String>,
) -> (StatusCode, Json<Movie>) {
    let db = db.lock().unwrap();
    match db.get(&id) {
        Some(movie) => (StatusCode::OK, Json(movie.clone())),
        None => (StatusCode::NOT_FOUND, Json(Movie::default()))
    }
}

async fn add_movie(
    State(db): State<Arc<Mutex<HashMap<String, Movie>>>>,
    Json(payload): Json<Movie>,
) -> (StatusCode, Json<Movie>) {
    let mut db = db.lock().unwrap();
    let movie = payload.clone();
    db.insert(movie.id.clone(), movie.clone());
    (StatusCode::CREATED, Json(movie))
}
