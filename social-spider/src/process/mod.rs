pub mod source;
pub mod post;

use hyper::Client;
use log::{info, error};

/// Downloads the body of a URL
pub async fn download_body(resp: hyper::Response<hyper::Body>) -> Option<String> {
    info!("RESP: {:?}", resp);

    if resp.status().is_client_error() {
        error!("Encountered a client error. {}", resp.status());
        return None;
    } else if resp.status().is_server_error() {
        error!("Encountered a server error. {}", resp.status());
        return None;
    }

    let bytes = hyper::body::to_bytes(resp.into_body()).await.expect("Failed to turn body to bytes.");
    let ret = Some(match String::from_utf8(bytes.to_vec()) {
        Ok(s) => s,
        Err(e) => { error!("Could not mutate bytes received into UTF-8. {}", e); return None; }
    });

    println!("{}", ret.clone().unwrap_or("NO BODY".to_string()));
    ret
}

/// Gets a response
pub async fn get_response(url: String) -> hyper::Response<hyper::Body> {
    let url = "https://twitter.com/i/api/graphql/Vwkjj-4-TWRBblnhvaXyhQ/TweetDetail?variables=%7B%22focalTweetId%22%3A%22".to_owned() + &url + "%22%2C%22with_rux_injections%22%3Afalse%2C%22includePromotedContent%22%3Atrue%2C%22withCommunity%22%3Atrue%2C%22withTweetQuoteCount%22%3Atrue%2C%22withBirdwatchNotes%22%3Afalse%2C%22withSuperFollowsUserFields%22%3Atrue%2C%22withUserResults%22%3Atrue%2C%22withBirdwatchPivots%22%3Afalse%2C%22withReactionsMetadata%22%3Afalse%2C%22withReactionsPerspective%22%3Afalse%2C%22withSuperFollowsTweetFields%22%3Atrue%2C%22withVoice%22%3Atrue%7D";
    let client = Client::builder().build::<_, hyper::Body>(hyper_tls::HttpsConnector::new());
    let uri = /* url */ url.parse::<hyper::Uri>().expect("Bad URL, failed to parse.");
    let request: hyper::Request<hyper::Body>;

    if url.contains("twitter.com") {
        // Trick Twitter into thinking we are them :D
        request = hyper::Request::builder().uri(uri)
            .header("dnt", "1")
            .header("x-twitter-client-language", "en")
            .header("sec-fetch-dest", "empty")
            .header("accept", "*/*")
            .header("origin", "https://twitter.com")
            .header("sec-fetch-site", "same-site")
            .header("sec-fetch-mode", "cors")
            .header("authorization", "Bearer AAAAAAAAAAAAAAAAAAAAANRILgAAAAAAnNwIzUejRCOuH5E6I8xnZz4puTs%3D1Zv7ttfk8LF81IUq16cHjhLTvJu4FA33AGWWjCpTnA")
            .header("user-agent", "Mozilla/5.0 (X11; Linux x86_64; rv:89.0) Gecko/20100101 Firefox/89.0")
            .header("referer", "https://twitter.com/".to_owned() + &url)
            .header("x-guest-token", "1449453354824785925")
            .header("x-twitter-active-user", "yes")
            .header("x-requested-with", "XMLHttpRequest")
            .header("accept-language", "en-US")
            .body(hyper::Body::empty()).unwrap();
    } else {
        request = hyper::Request::builder().uri(uri).body(hyper::Body::empty()).unwrap();
    }

    client.request(request).await.unwrap()
}

