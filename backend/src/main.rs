use actix::prelude::*;
use actix_files::NamedFile;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use dotenv::dotenv;
use std::path::PathBuf;

async fn index(_req: HttpRequest) -> actix_web::Result<NamedFile> {
    let path: PathBuf = "./static/index.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

use tiletapper::client::Client;
use tiletapper::server;

async fn new_websocket_connection(
    r: HttpRequest,
    stream: web::Payload,
    server_addr: web::Data<Addr<server::Server>>,
) -> Result<HttpResponse, Error> {
    ws::start(Client::new(server_addr.get_ref().clone(), None), &r, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("starting HTTP server at http://localhost:9001");

    // creating room
    let server = server::Server::new().start();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(server.clone()))
            .route("/ws", web::get().to(new_websocket_connection))
            .route("/{filename:.*}", web::get().to(index))
            .wrap(actix_web::middleware::Logger::default())
    })
    .workers(2)
    .bind(("127.0.0.1", 9001))?
    .run()
    .await
}
