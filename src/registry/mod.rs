pub mod registry;

use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter, Pointer};
use serde::{Serialize, Deserialize};


pub type IResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Serialize, Deserialize, Clone)]
pub struct Service {
    pub name: String,
    pub version: String,
    pub metadata: HashMap<String, String>,
    pub endpoint: Vec<String>,
    pub nodes: Vec<Node>,
}

impl Service {
    pub fn new(name: String, version: String, metadata: HashMap<String, String>, endpoint: Vec<String>, nodes: Vec<Node>) -> Self {
        Service {
            name,
            version,
            metadata,
            endpoint,
            nodes,
        }
    }

    pub fn to_json_string(&self) -> IResult<String> {
        let s = serde_json::to_string(self)?;
        Ok(s)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Node {
    pub id: String,
    pub address: String,
    pub metadata: HashMap<String, String>,
}

impl Node {
    pub fn new(id: String, address: String, metadata: HashMap<String, String>) -> Self {
        Self {
            id,
            address,
            metadata,
        }
    }
}
