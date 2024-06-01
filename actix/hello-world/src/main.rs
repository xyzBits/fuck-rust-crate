use actix_web::{App, delete, get, HttpResponse, HttpServer, post, put, Responder, web};
use serde::{Deserialize, Serialize};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the index page!")
}

#[post("/create")]
async fn create() -> impl Responder {
    HttpResponse::Ok().body("Resource created successfully!")
}

#[put("/update")]
async fn update() -> impl Responder {
    HttpResponse::Ok().body("Resource updated successfully!")
}

#[delete("/delete")]
async fn delete() -> impl Responder {
    HttpResponse::NoContent().finish()
}


#[get("/user/{id}/{name}")]
async fn user_info(info: web::Path<(u32, String)>) -> impl Responder {
    let (id, name) = info.into_inner();
    HttpResponse::Ok().body(format!("User ID: {}, Name: {}", id, name))
}

#[derive(Debug, Deserialize, Serialize)]
struct User {
    username: String,
    email: String,
}

/// ```shell
/// curl --header "Content-Type: application/json" \
/// --request POST \
/// --data '{"username":"xyz","email":"xyz@gmail.com"}' \
/// http://localhost:8080/create_user
/// ```
#[post("/create_user")]
async fn create_user(user: web::Json<User>) -> impl Responder {
    let new_user = user.into_inner();

    // Process the new user data (e.g. save it into a database)
    HttpResponse::Created().json(new_user)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(create)
            .service(update)
            .service(delete)
            .service(user_info)
            .service(create_user)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
