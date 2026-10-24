use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use routes::health_check::health_check;
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/subscribe", web::get().to(subscribe))
            .route("/health", web::get().to(health_check))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
