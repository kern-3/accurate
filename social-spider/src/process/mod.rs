pub mod source;
pub mod post;

use hyper::Client;
use log::error;

/// Downloads the body of a URL
pub async fn download_body(url: String) -> Option<String> {
    let client = Client::new();
    let uri = url.parse::<hyper::Uri>().expect("Bad URL, failed to parse.");
    let resp = client.get(uri).await.expect("Failed to get URL.");

    if resp.status().is_client_error() {
        error!("Encountered a client error. {}", resp.status());
        return None;
    } else if resp.status().is_server_error() {
        error!("Encountered a server error. {}", resp.status());
        return None;
    }

    let bytes = hyper::body::to_bytes(resp.into_body()).await.expect("Failed to turn body to bytes.");
    Some(match String::from_utf8(bytes.to_vec()) {
        Ok(s) => s,
        Err(e) => { error!("Could not mutate bytes received into UTF-8. {}", e); return None; }
    })
}

