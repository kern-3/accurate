#[allow(unused_imports)]

use std::convert::Infallible;
use std::net::SocketAddr;
use std::os::unix::net::UnixStream;
use std::io::prelude::*;

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

            // Run the sentiment analysis (IPC to tensortalk over unix sockets)
            debug!("Connecting to tensortalk");
            let mut ipc_stream = match UnixStream::connect("/tmp/tensortalk-accurate.s") { 
                Ok(s) => s,
                Err(e) => {
                    error!("Failed to connect to tensortalk unix socket. Is it runing? {}", e);
                    return Ok(Response::builder().status(StatusCode::INTERNAL_SERVER_ERROR).body(Body::from("Tensortalk connection failed")).unwrap());
                }
            };

            debug!("Querying tensortalk");
            match ipc_stream.write_all(uri.q.as_bytes()) {
                Ok(()) => (),
                Err(e) => {
                    error!("Failed to write to tensortalk unix socket. {}", e);
                    return Ok(Response::builder().status(StatusCode::INTERNAL_SERVER_ERROR).body(Body::from("Tensortalk write failed")).unwrap());
                }
            };

            debug!("Listening to tensortalk");
            let mut string_accuracy = String::new();
            match ipc_stream.read_to_string(&mut string_accuracy) {
                Ok(_) => (),
                Err(e) => {
                    error!("Failed to read from tensortalk unix socket. {}", e);
                    return Ok(Response::builder().status(StatusCode::INTERNAL_SERVER_ERROR).body(Body::from("Tensortalk read failed")).unwrap());
                }
            };

            // Calculate percentage
            let float_accuracy: f64 = string_accuracy.parse().unwrap();
            let percent_accuracy: u8 = (float_accuracy * 100.0) as u8;

            // Respond
            info!("Rating: {}", percent_accuracy);
            let response = serde_json::to_string(&QueryResponse::new(percent_accuracy)).unwrap();
            Ok(Response::builder().status(StatusCode::OK).body(Body::from(response)).unwrap())
        },

        // Return the 404 Not Found for other routes.
        _ => {
            Ok(Response::builder().status(StatusCode::NOT_FOUND).body(Body::from("supply an endpoint")).unwrap())
        },
    }
}

