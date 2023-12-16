use actix_files as fs;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(fs::Files::new("/", "./static")))
        .bind(("0.0.0.0", 8000))?
        .run()
        .await
}
