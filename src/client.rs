
use actix::*;
use actix_codec::Framed;
use actix::io::SinkWrite;

use futures::stream::SplitSink;

use awc::{
    BoxedSocket,
    error::WsProtocolError,
    ws::{Codec, Frame, Message},
};

pub struct ClientActor(pub SinkWrite<Message, SplitSink<Framed<BoxedSocket,Codec>,Message>>);

impl Actor for ClientActor {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Context<Self>){
        println!("Started the Client Actor");
    }

}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientCommand(pub String);

impl Handler<ClientCommand> for ClientActor {
    type Result = ();

    fn handle(&mut self, msg: ClientCommand, _: &mut Context<Self>) {
        self.0.write(Message::Text(msg.0)).unwrap();
    }
}

impl StreamHandler<Result<Frame, WsProtocolError>> for ClientActor {
    fn handle(&mut self, msg:Result<Frame,WsProtocolError>, _: &mut Self::Context) {
        match msg {
            Ok(frame) => match frame {
                Frame::Text(txt) => println!("Server Text: {:?}", txt),
                _ => ()
            }
            Err(e) => {
            println!("there was an error: {:?}",e);
                ()
            }
        }

    }
}

impl actix::io::WriteHandler<WsProtocolError> for ClientActor{}