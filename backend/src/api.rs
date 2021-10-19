use std::convert::Infallible;
use std::net::SocketAddr;

use serde::{Serialize, Deserialize};
use log::{debug, info, error};
use hyper::{Method, StatusCode, Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use clap::ArgMatches;

#[derive(Debug, Deserialize)]
pub struct Query {
    pub q: String,
}

#[derive(Debug, Serialize)]
pub struct QueryResponse {
    pub p: u8,
}

impl QueryResponse {
    pub fn new(p: u8) -> Self {
        Self { p }
    }
}

pub async fn serve<'a>(args: &'a ArgMatches) {
    let host = args.value_of("host").expect("host value came back empty. unreachable.");
    let bind_on = match host.parse() {
        Ok(b) => b,
        Err(e) => { error!("Failed to parse hot to serve on {}. {}", host, e); return; }
    };

    debug!("Assembling service");
    let service = make_service_fn(move |_| {
        async move {
            // async block is only executed once, so just pass it on to the closure
            Ok::<_, hyper::Error>(service_fn(move |req| {
                async move { client_dispatch(req).await }
            }))
        }
    });

    debug!("Binding");
    let server = match Server::try_bind(&bind_on) {
        Ok(s) => s,
        Err(e) => { error!("Failed to bind on {}. {}", &bind_on, e); return; }
    }.serve(service);

    info!("Serving!");
    match server.await {
        Ok(()) => (),
        Err(e) => {
            error!(
                "The server (Hyper) encountered a error, mistake, misconception, delusion, inaccuracy, miscalculation, blunder, fault, flaw, oversight, or misprint. {}", e);
            return;
        },
    };
}

async fn client_dispatch(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/query") => {
            let uri: Query = match serde_qs::from_str(req.uri().query().unwrap_or("")) {
                Ok(u) => u,
                Err(e) => {
                    error!("Failed to parse query string. {}", e);
                    return Ok(Response::builder().status(StatusCode::BAD_REQUEST).body(Body::from("Bad request")).unwrap());
                }
            };

            debug!("Got OK request (in theory): {:?}", uri);

            // Run the sentiment analysis

            // Respond
            let response = serde_json::to_string(&QueryResponse::new(42)).unwrap();
            Ok(Response::builder().status(StatusCode::OK).body(Body::from(response)).unwrap())
        },

        // Return the 404 Not Found for other routes.
        _ => {
            Ok(Response::builder().status(StatusCode::NOT_FOUND).body(Body::from("supply an endpoint")).unwrap())
        },
    }
}

