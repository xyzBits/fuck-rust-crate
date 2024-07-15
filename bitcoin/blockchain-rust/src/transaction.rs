use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct  Transaction {

}

impl Transaction {
    pub fn get_id(&self) -> &[u8] {
        todo!()
    }
}