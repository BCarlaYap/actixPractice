use actix::{Actor, StreamHandler};
use actix_web_actors::ws;
use actix_web_actors::ws::WebsocketContext;


impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for SessionActor{
    fn handle(&mut self, msg: Result<ws::Message,ws::ProtocolError>, _: &mut Self::Context){
        println!("received message: {:?}",msg);
    }

    fn started (&mut self, _: &mut Self::Context){
        println!("Server actor has started");
    }

}

pub struct SessionActor;

impl Actor for SessionActor{
    type Context = WebsocketContext<Self>;
}