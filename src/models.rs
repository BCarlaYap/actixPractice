use serde::{Serialize,Deserialize};


#[derive(Serialize,Deserialize)]
pub struct AddIP{
    pub ip_address:String
}

#[derive(Serialize,Deserialize)]
pub struct RemoveIP{
    pub ip_address:String
}

pub struct GetWhiteList;

pub struct GetLocalList;

#[derive(Serialize,Deserialize)]
pub struct ConfirmSecretKey{
    pub secret_key:String
}
