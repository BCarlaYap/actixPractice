use actix::*;
use actix_rt::Arbiter;

use awc::Client;

/*
 *  To forcefully fail this client to connect.
*/
fn main() {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    let sys = System::new("client");

    Arbiter::spawn(async {
         match Client::new().ws("ws://0.0.0.0:8080/failed/").connect().await {
             Ok(_) => println!("It's not suppose to be success"),
             Err(_) => {
                 println!("unauthorized access");
                 std::process::exit(0);
             }
         }
    });

 sys.run().unwrap();
}