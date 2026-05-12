use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload {
    pub hostname: String,
    pub data: PayloadData,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PayloadData {
    Hello,
}