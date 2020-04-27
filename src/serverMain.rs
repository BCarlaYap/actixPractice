mod models;
mod config;
mod server;
mod session;
mod handlers;

use crate::handlers::*;
use crate::config::Config;

use std::io;
use dotenv::dotenv;
use futures::future;

use actix::Actor;
use actix_web::{HttpServer, App,web, middleware};


#[actix_rt::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    dotenv().ok();
    let config = Config::from_env().unwrap();

    let server_addr = server::ServerActor::new().start();
    let server_addr_clone = server_addr.clone();

    println!("server started http://{}:{}",config.server.host, config.server.port);
    let server1 = HttpServer::new(move||{
        App::new()
            .data(server_addr.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/connect/{_}").route(web::get().to(connect_with_secret)))
            .service(web::resource("/success/").route(web::get().to(connect_ws)))
            .service(web::resource("/fail/").route(web::get().to(fail_connect_ws)))
            .route("/whitelist{_:/?}", web::post().to(add_ip))
            .route("/whitelist{_:/?}", web::delete().to(delete_ip))
            })
        .bind(format!("{}:{}", config.server.host,config.server.port))?
        .run();

    let server2 = HttpServer::new(move ||{
        App::new()
            .data(server_addr_clone.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/whitelist{_:/?}").route(web::get().to(get_list)))
    }).bind(format!("{}:{}", config.server.host,config.server.port_for_display))?
        .run();


    future::try_join(server1,server2).await?;
    Ok(())
}
