use actix_web::{dev::Server, web, App, HttpResponse, HttpServer, Responder};

// Notice the different signature!
// We return `Server` on the happy path and we dropped the `async` keyword
// We have no .await call, so it is not needed anymore.
pub fn run(address: &str) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .bind(address)?
        .run();
    // No .await here!
    Ok(server)
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
