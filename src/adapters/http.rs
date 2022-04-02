use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::server::conn::AddrStream;
use std::sync::{Arc, Mutex};

use crate::WasmDbEngine;

pub struct HttpAdapter {}

impl HttpAdapter {
    pub async fn start(&mut self, engine: Arc<Mutex<WasmDbEngine>>) {
        let context = AppContext {
            engine
        };

        // We'll bind to 0.0.0.0:3000
        let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

        // A `Service` is needed for every connection, so this
        // creates one from our `hello_world` function.
        // A `MakeService` that produces a `Service` to handle each connection.
        let make_service = make_service_fn(move |conn: &AddrStream| {
            // We have to clone the context to share it with each invocation of
            // `make_service`. If your data doesn't implement `Clone` consider using
            // an `std::sync::Arc`.
            let context = context.clone();

            // You can grab the address of the incoming connection like so.
            let addr = conn.remote_addr();

            // Create a `Service` for responding to the request.
            let service = service_fn(move |req| handle(context.clone(), addr, req));

            // Return the service to hyper.
            async move { Ok::<_, Infallible>(service) }
        });

        let server = Server::bind(&addr).serve(make_service);

        // Run this server for... forever!
        if let Err(e) = server.await {
            eprintln!("server error: {}", e);
        }
    }
}

#[derive(Clone)]
struct AppContext{
    engine: Arc<Mutex<WasmDbEngine>>,
}

async fn handle(
    context: AppContext,
    _: SocketAddr,
    req: Request<Body>,
) -> Result<Response<Body>, Infallible> {

    let path = req.uri().path();
    let mut parts: Vec<&str> = path.split("/").collect();

    parts.remove(0);

    if parts.len() < 1 || parts[0].len() == 0 {
        return Ok(Response::new(Body::from("Missing func name, call with /<func_name>/<arg1>/<arg2>/...")));
    }

    let func_name = parts[0];
    let args = parts[1..].to_vec().iter().map(|x| x.to_string()).collect();

    let response = context.engine.lock().unwrap().run(func_name.to_string(), args).unwrap();

    Ok(Response::new(Body::from(format!("{:?}", response))))
}
