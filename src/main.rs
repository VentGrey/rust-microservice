extern crate hyper;
extern crate futures;

#[macro_use]
extern crate log;
extern crate env_logger;

use hyper::Request;
use hyper::Response;
use hyper::service::Service;

use futures::future::Future;

struct Microservice;

impl Service for Microservice {
    type ReqBody = Self::ReqBody;    //type Request = Request;
    type ResBody = Self::ResBody;    //type Response = Response;
    type Error = hyper::Error;
    type Future = Box<dyn Future<Item = Self::ResBody, Error = Self::Error>>;

    fn call(&self, request: Self::ReqBody) -> Self::Future {
        info!("Microservice received a request: {:?}", request);
        Box::new(futures::future::ok(Response::new()))
    }
}
