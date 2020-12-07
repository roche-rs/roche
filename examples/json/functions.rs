use serde::{Deserialize, Serialize};
use tide::prelude::*; // Pulls in the json! macro.


#[derive(Deserialize, Serialize)]
struct Cat {
    name: String,
}

pub fn handler() -> tide::Server<()> {    
    let mut api = tide::new();
    api.at("/animals").get(|_| async {
        Ok(json!({
            "meta": { "count": 2 },
            "animals": [
                { "type": "cat", "name": "chashu" },
                { "type": "cat", "name": "nori" }
            ]
        }))
    });
    api
}