mod actor;
mod errors;
mod session_manager;

use actix::prelude::*;
use actix_web::web::Data;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Error};
use actix_web_actors::ws;
use crate::errors::ConnectionError;
use crate::actor::WebSocket;
use crate::session_manager::WsSessionManager;

async fn ws_index(req: HttpRequest, stream: web::Payload, server_instance: web::Data<Addr<WsSessionManager>>)
    -> Result <HttpResponse, Error> {

        // create websocket actor instance

        let ws_actor = WebSocket {
            manager: server_instance.get_ref().clone(),
        };

        ws::start(ws_actor, &req, stream)
    }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Create and start the WsSessionManager Actor
    let manager = WsSessionManager::new().start();
    // Specify the default address for our web app
    let address = "0.0.0.0:8080";
    // Construct the Actix web app
    let server = HttpServer::new(move || {
        App::new()
            // WebSocket route
            .route("/ws/", web::get().to(ws_index))
            // Serving static file
            .service(actix_files::Files::new("/", "public").index_file("index.html"))
            // Share the manager actor across the application
            .app_data(Data::new(manager.clone()))
    })
    .workers(1)
    .bind(address);

    match server {
        Ok(srv) => {
            println!("Server listening on: {}", address);
            srv.run().await
        },
        Err(err) => {
            panic!("{}", ConnectionError::CreateServerError(err.to_string()))
        }
    }

}
    
