use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Custom {
    pub a: u64,
    pub str: String,
}
