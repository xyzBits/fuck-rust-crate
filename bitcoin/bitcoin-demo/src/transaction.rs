use serde::{Deserialize, Serialize};

use super::*;
use crate::errors::*;

#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct Transaction {

}


impl Transaction {
    pub fn hash(&self) -> Result<String> {
        todo!()
    }
}