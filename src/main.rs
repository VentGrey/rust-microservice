#[macro_use]
extern crate log;
extern crate env_logger;

use hyper::server::{Request, Response, Service};

use futures::future::Future;

struct Microservice;

impl Service for Microservice {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<dyn Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, request: Request) -> Self::Future {
        info!("El microservicio recibió la petición: {:?}", request);
        Box::new(futures::future::ok(Response::new()))
    }
}

fn main() {
    env_logger::init();

    let addr = "127.0.0.1:8000".parse().unwrap();
    let server = hyper::server::Http::new()
        .bind(&addr, || Ok(Microservice{}))
        .unwrap();

    info!("Ejecutando microservicio en: {}", addr);
    server.run().unwrap();
}
