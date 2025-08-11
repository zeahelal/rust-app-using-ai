use anyhow::Result;
use reqwest::Client;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::app::Message;

const API_BASE_URL: &str = "http://localhost:8080";

pub fn fetch_hello(callback: Callback<Result<String>>) {
    spawn_local(async move {
        let url = format!("{}/api/hello", API_BASE_URL);
        let response = Client::new().get(&url).send().await;
        
        let result = match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    resp.text().await.map_err(|e| anyhow::anyhow!(e.to_string()))
                } else {
                    Err(anyhow::anyhow!("Request failed with status: {}", resp.status()))
                }
            }
            Err(e) => Err(anyhow::anyhow!(e.to_string())),
        };
        
        callback.emit(result);
    });
}
