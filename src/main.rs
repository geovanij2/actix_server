use actix_web::{get, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| App::new().service(hello))
        .bind("localhost:8000")?
        .run();

    println!("Server running at http://{}/", "localhost:8000");

    server.await
}

#[get("/")]
async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello World")
}
