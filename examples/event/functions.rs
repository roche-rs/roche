use serde::{Deserialize, Serialize};
use tide::http::mime;
use tide::prelude::*; // Pulls in the json! macro.
use tide::{Body, Response};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
struct Cat {
    name: String,
}

pub fn handler() -> tide::Server<()> {
    let mut api = tide::new();
    api.at("/").post(|_| async {
        let msguuid = Uuid::new_v4();
        let json = json!({
        "msg": "Hi from rust app!"
        });
        let body = Body::from_json(&json)?;

        let response = Response::builder(203)
            .body(body)
            .header("Ce-Id", msguuid.to_string())
            .header("Ce-specversion", "0.3")
            .header("Ce-Source", "knative/eventing/samples/hello-world")
            .header("Ce-Type", "dev.knative.samples.hifromknative")
            .content_type(mime::JSON)
            .build();
        Ok(response)
    });
    api
}
