use ::chess::*;
use actix::*;
use actix_files::Files;
use actix_web::*;
use actix_web_actors::ws;
use std::time::Instant;

async fn ws_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<server::Server>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        session::WsSession {
            id: 0,
            hb: Instant::now(),
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = server::Server::default().start();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(server.clone()))
            .route("/ws", web::get().to(ws_route))
            .service(web::redirect("/", "/index.html"))
            .service(Files::new("/", "./static"))
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
