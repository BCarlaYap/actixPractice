pub mod client;

use crate::client::{ClientActor,ClientCommand};

use actix::*;
use actix_rt::Arbiter;
use actix::io::SinkWrite;

use awc::Client;
use std::{io,thread};
use futures::StreamExt;

/*
 *  Connects to the websocket; and if valid, can send a text inputted from the terminal.
*/
fn main()  {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    let sys = System::new("client");

    Arbiter::spawn(async {
        let args: Vec<String> = std::env::args().collect();

        Client::new()
            .ws(format!("ws://0.0.0.0:8080/connect/{}",args[1]))
            .connect()
            .await
            .map_err(|e| {
                println!("{}", e);
            })
            .map(|(response,framed)|{
                println!("{:?}",response);

                let (sink, stream) = framed.split();
                let addr = ClientActor::create(|ctx| {
                    ClientActor::add_stream(stream, ctx);
                    ClientActor(SinkWrite::new(sink, ctx))
                });

                // start console loop
                thread::spawn(move || loop {
                    let mut cmd = String::new();

                    if io::stdin().read_line(&mut cmd).is_err() {
                        println!("error");
                        return;
                    }

                    addr.do_send(ClientCommand(cmd.trim().to_string()));
                });

        }).unwrap();
    });

    sys.run().unwrap();
}

