use std::sync::Arc;

use axum::{
    extract::State, http::StatusCode, routing::{get, post}, Json, Router
};
use serde::{Deserialize, Serialize};



#[derive(Debug)]
struct HttpServeState {
    password: String,
}



#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();
    let state = HttpServeState {
        password: "hello axum".to_string(),
    };

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user))
        .with_state(Arc::new(state));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    State(state): State<HttpServeState>,
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {

    println!("{:?}", state);

    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
