

use crate::session::*;
use crate::server;
use crate::models::{AddIP, RemoveIP,GetWhiteList,GetLocalList};

use std::collections::HashMap;

use actix_web_actors::ws;
use actix_web::{web, Responder, HttpResponse, HttpRequest};
use actix::{Addr, MailboxError};


pub async fn connect_ws(req: HttpRequest,
                        stream: web::Payload,
                        server_data: web::Data<Addr<server::ServerActor>>
) -> Result<impl Responder, MailboxError> {

    server_data.get_ref().send(GetWhiteList).await
        .map(|res|{
            match res {
                Ok(whitelist) => {
                    println!("whitelist: {:?}",whitelist);

                    let ip_addr = match req.head().peer_addr {
                        Some(soc_addr) => soc_addr.ip().to_string(),
                        _ => "0.0.0.0".to_string(),
                    };

                    if whitelist.contains_key(&ip_addr) {
                        println!("ip address valid: {:?} ", ip_addr);
                        ws::start(SessionActor,&req,stream)
                    }
                    else {
                        println!("ip address not valid: {:?}",ip_addr);
                        Ok(HttpResponse::InternalServerError().json("error"))
                    }

                },

                Err(e) =>  {
                    println!("Error on GetWhiteList: {:?}",e);
                    Ok(HttpResponse::InternalServerError().json("error"))

                }
            }

        })
}


pub async fn fail_connect_ws(req: HttpRequest ) -> impl Responder {
    let ip_addr = match req.head().peer_addr {
        Some(soc_addr) => soc_addr.ip().to_string(),
        _ => "0.0.0.0".to_string(),
    };
    HttpResponse::InternalServerError().json(format!("{} not authorized.",ip_addr))
}


pub async fn get_list(server_data: web::Data<Addr<server::ServerActor>>) -> impl Responder {
    server_data.get_ref().send(GetWhiteList).await
        .map(|res|{
            match res{
                Ok(whitelist) => HttpResponse::Ok().json(whitelist),
                Err(_) => HttpResponse::Ok().json("no list found")
            }
        })
}


/*
 * Only local ip addresses can do this operation: to remove ip addresses from the whitelist
 * {
 *    "ip_address": "192.168.0.15"
 * }
*/
pub async fn delete_ip(req: HttpRequest,
                       server_data: web::Data<Addr<server::ServerActor>>,
                       json: web::Json<AddIP>
) -> impl Responder {

    let ip_addr = get_ip(req);
    let server_ref = server_data.get_ref();
    match get_local_list(server_ref).await{
        Ok(local_list) => {
            if local_list.contains_key(&ip_addr.await.clone()) {
                match server_ref.send(RemoveIP { ip_address: json.ip_address.clone() }).await {
                    Ok(send_resp) => HttpResponse::Ok().json(send_resp),
                    Err(_) => HttpResponse::InternalServerError().json("failed to send AddIp command:{}")
                }
            } else {
                HttpResponse::InternalServerError().json("key not found")
            }
        }
        Err(_) => {
            HttpResponse::InternalServerError().json("failed to get local list")
            }
        }
    }


/*
 * Only local ip addresses can do this operation: to add allowable ip addresses
 * {
 *    "ip_address": "192.168.0.15"
 * }
*/
pub async fn add_ip(req: HttpRequest,server_data: web::Data<Addr<server::ServerActor>>, json: web::Json<AddIP>)
 -> impl Responder {

    let ip_addr = get_ip(req);
    let server_ref = server_data.get_ref();

   match get_local_list(server_ref).await{
        Ok(local_list) => {
            if local_list.contains_key(&ip_addr.await.clone()) {
                match server_ref.send(AddIP{ ip_address: json.ip_address.clone() }).await {
                    Ok(send_resp) => HttpResponse::Ok().json(send_resp),
                    Err(_) => HttpResponse::InternalServerError().json("failed to send AddIp command:{}")
                }
            } else {
                HttpResponse::InternalServerError().json("failed to get local list")
            }
        }

       Err(_) => HttpResponse::InternalServerError().json(format!("ip address:{} not found",&ip_addr.await.clone()))
    }
}


async fn get_ip(req: HttpRequest) -> String {
    let ip_addr_ref = req.connection_info();
    let ip_addr = ip_addr_ref.remote().unwrap_or("").split(":").collect::<Vec<&str>>();

    ip_addr[0].to_string()
}


async fn get_local_list(server_actor:&Addr<server::ServerActor>)
    -> Result<HashMap<String,bool>,MailboxError> {
    server_actor.send(GetLocalList).await
       .map(|res|{
        match res{
            Ok(local_list) => local_list,
            Err(_) => HashMap::new()
        }
    })

}


