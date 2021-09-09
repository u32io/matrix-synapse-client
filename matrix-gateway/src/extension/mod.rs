use serde::de::DeserializeOwned;
use actix_web::client::ClientResponse;
use std::io::BufReader;

pub async fn get_json_body<T>(response: &mut ClientResponse) -> Option<T> where T : DeserializeOwned
{
    let bytes = response.body().await.ok()?;
    serde_json::from_slice(&*bytes).ok()
}