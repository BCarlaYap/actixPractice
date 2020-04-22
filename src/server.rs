use crate::models::{AddIP, RemoveIP, GetWhiteList, GetLocalList};

use std::io;
use std::collections::HashMap;

use actix::Actor;
use actix::prelude::*;


/*
 * message and handler when AddIP is invoked
*/
impl Message for AddIP{
    type Result = bool;
}

impl Handler<AddIP> for ServerActor{
    type Result = bool;

    fn handle(&mut self, msg:AddIP, _: &mut Self::Context) -> Self::Result {
        println!("received AddIP InternalCommand: {}", msg.ip_address);
        self.whitelist.insert(msg.ip_address.clone(),true);
        true
    }
}


/*
 * message and handler when RemoveIP is invoked
*/
impl Message for RemoveIP{
    type Result = bool;
}

impl Handler<RemoveIP> for ServerActor{
    type Result = bool;

    fn handle(&mut self, msg:RemoveIP, _: &mut Self::Context) -> Self::Result {
        println!("received RemoveIP InternalCommand: {}", msg.ip_address);
        self.whitelist.remove(msg.ip_address.as_str());
        true
    }
}


/*
 * GetWhiteList gets a list of ip addresses that can connect to the websocket
*/

impl Message for GetWhiteList{
    type Result = Result<HashMap<String,bool>, io::Error>;
}

impl Handler<GetWhiteList> for ServerActor{
    type Result = Result<HashMap<String,bool>, io::Error>;

    fn handle(&mut self, _:GetWhiteList, _: &mut Self::Context) -> Self::Result {
        println!("received GetWhiteList message");
       Ok(self.whitelist.clone())

    }
}


/*
 * GetLocalList gets a list of ip addresses binded to the localhost
*/
impl Message for GetLocalList{
    type Result = Result<HashMap<String,bool>, io::Error>;
}

impl Handler<GetLocalList> for ServerActor{
    type Result = Result<HashMap<String,bool>, io::Error>;

    fn handle(&mut self, _:GetLocalList, _: &mut Self::Context) -> Self::Result {
        println!("received GetLocalList message");
        Ok(self.local_list.clone())

    }
}


pub struct ServerActor{
    whitelist: HashMap<String,bool>,
    local_list:HashMap<String,bool>
}


impl Actor for ServerActor{
    type Context = Context<Self>;
}


impl ServerActor{
    pub fn new() -> Self {
        let mut default_white_list_map:HashMap<String,bool> = HashMap::new();
       default_white_list_map.insert("0.0.0.0".to_string(),true);
       default_white_list_map.insert("127.0.0.1".to_string(),true);

        let mut default_local_list_map:HashMap<String,bool> = HashMap::new();
       default_local_list_map.insert("0.0.0.0".to_string(),true);
       default_local_list_map.insert("127.0.0.1".to_string(),true);


        Self{
            whitelist:default_white_list_map,
            local_list:default_local_list_map,
        }
    }

}

