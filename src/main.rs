extern crate futures;
extern crate hyper;
extern crate rand;

use rand::random;
use futures::future;
use hyper::rt::{Future, Stream};
use hyper::service::{service_fn, service_fn_ok};
use hyper::{Body, Method, Request, Response, Server, StatusCode, Client};
use std::collections::HashMap;
use std::sync::Arc;

//Extended from the examples at https://github.com/hyperium/hyper/tree/master/examples
type BoxFut = Box<Future<Item = Response<Body>, Error = hyper::Error> + Send>;

fn forward(req: Request<Body>, forward_table: Arc<HashMap<(String, String), Vec<hyper::Uri>>>) -> Response<Body> {
    let client = Client::new();
    match forward_table.get(&(req.method().to_string(), req.uri().path().to_string())){
        Some(entry) => {
            let mut request = Request::builder().method(req.method())
                .uri(entry.get(random::<usize>()%entry.len()).unwrap()) //Pick random endpoint to service request.
                .header("X-Custom-Foo", "Bar")
                .body(req.into_body())
                .unwrap();
            client.request(request);
        },
        None => { //No endpoint found. Send to default route
            let mut request = Request::builder().method(req.method())
                .uri("http://httpbin.org/ip")
                .header("X-Custom-Foo", "Bar")
                .body(req.into_body())
                .unwrap();
            client.request(request);
        },
    }
    Response::new(Body::from(format!("Request ")))
}

fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();
    //Create and fill forwarding table later
    let forward_table: HashMap<(String, String), Vec<hyper::Uri>> = HashMap::new();
    let arc_table = Arc::new(forward_table);

    let server = Server::bind(&addr)
        .serve(move || {
            let arc_table = arc_table.clone();
            service_fn_ok(move |req| forward(req, arc_table.clone()))
        })
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", addr);
    hyper::rt::run(server);
}
