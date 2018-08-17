extern crate futures;
extern crate hyper;

use futures::future;
use hyper::rt::{Future, Stream};
use hyper::service::{service_fn, service_fn_ok};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::collections::HashMap;
use std::sync::Arc;

//Extended from the examples at https://github.com/hyperium/hyper/tree/master/examples
type BoxFut = Box<Future<Item = Response<Body>, Error = hyper::Error> + Send>;

fn test(req: Request<Body>, forward_table: Arc<HashMap<(String, String), hyper::Uri>>) -> Response<Body> {
    Response::new(Body::from(format!("Request ")))
}

fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();
    let forward_table: HashMap<(String, String), hyper::Uri> = HashMap::new();
    let arc_table = Arc::new(forward_table);
    //
    // let service_forward = move || {
    //     let arc_table = arc_table.clone();
    //     service_fn_ok(move |req: Request<Body>| {
    //         let handler = arc_table.entry((req.method().to_string(), req.uri().path().to_string()));
    //         Response::new(Body::from(format!("Request ")))
    //     })
    // };

    let server = Server::bind(&addr)
        //.serve(|| service_fn(service_forward))
        .serve(|| service_fn_ok(|req| test(req, arc_table.clone())))
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", addr);
    hyper::rt::run(server);
}
