use actix_web::{web, App, HttpServer, HttpResponse};

// Handler for GET requests
async fn get_hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello, World!")
}

// Handler for POST requests
async fn post_hello(item: web::Json<String>) -> HttpResponse {
    HttpResponse::Ok().body(format!("Received: {}", item))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(get_hello))
            .route("/hello", web::post().to(post_hello))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
