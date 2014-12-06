extern crate hyper;

use http;
use http::{Request, Response};
use uri::{Uri};
use std::sync::{Arc};

pub trait RequestHandler: Sync + Send {
  fn handle_request(&self, req: Request) -> Response;
}

pub struct Server {
  address: String,
  port: u16
}

impl Server {
  pub fn new(address: &str, port: u16) -> Server {
    Server { address: address.to_string(), port: port }
  }

  pub fn listen<H: RequestHandler>(&self, handler: H) {
    println!("Listening at {}:{}...", self.address, self.port);
    let server = hyper::Server::http(from_str(self.address.as_slice()).unwrap(), self.port);
    let sh = SecureHandler { handler: Arc::new(handler) };
    server.listen(sh).unwrap();
  }
}

#[deriving(Clone)]
struct SecureHandler<H: RequestHandler> {
  handler: Arc<H>
}

fn write_res_to_hres(res: Response, mut hres: hyper::server::Response<hyper::net::Fresh>) {
  *hres.status_mut() = match res.status {
    http::Status::Ok => hyper::Ok,
    http::Status::NotFound => hyper::NotFound,
  };

  hres.start().and_then(|mut r| {
    try!(r.write_str(res.body.as_slice()));
    r.end()
  }).unwrap();
}

fn hmethod_to_method(hmeth: hyper::method::Method) -> http::Method {
  match hmeth {
    hyper::Get => http::Method::GET,
    hyper::Post => http::Method::POST,
    _ => http::Method::GET // TODO
  }
}

fn huri_to_uri(huri: hyper::uri::RequestUri) -> Uri {
  match huri {
    hyper::uri::RequestUri::AbsolutePath(s) => Uri { path: s },
    hyper::uri::RequestUri::AbsoluteUri(url) => Uri { path: url.to_string() },
    hyper::uri::RequestUri::Authority(s) => Uri { path: s },
    hyper::uri::RequestUri::Star => Uri { path: "*".to_string() }
  }
}

fn hreq_to_req(hreq: hyper::server::Request) -> Request {
  Request {
    method: hmethod_to_method(hreq.method),
    uri: huri_to_uri(hreq.uri)
    // TODO: More
  }
}

impl<H: RequestHandler> hyper::server::Handler for SecureHandler<H> {
  fn handle(&self, hreq: hyper::server::Request, hres: hyper::server::Response<hyper::net::Fresh>) {
    let h = self.handler.deref();
    let request = hreq_to_req(hreq);
    let response = h.handle_request(request);
    write_res_to_hres(response, hres)
  }
}
