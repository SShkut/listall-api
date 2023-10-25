use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpResponse, HttpServer};

#[derive(serde::Deserialize)]
struct ProductFormData {
    name: String,
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn create_product(_form: web::Form<ProductFormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/products", web::post().to(create_product))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
