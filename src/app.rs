use routes;
use routes::{Responder};
use http;
use server::{Server, RequestHandler};

use std::collections::{HashMap};

pub struct App {
  routes: HashMap<String, Box<Responder>>
}

impl App {
  pub fn new() -> App {
    App { routes: HashMap::new() }
  }

  pub fn listen(self, address: &str, port: u16) {
    let server = Server::new(address, port);
    server.listen(self);
  }

  pub fn get(&mut self, pathspec: &str, r: &routes::ToResponder) {
    let responder = r.to_responder();
    self.routes.insert(pathspec.to_string(), responder);
  }

  pub fn debug_get(&self, path: &str) {
    let request = http::Request::new(http::Method::GET, path);
    self.debug_request(request);
  }

  pub fn debug_request(&self, request: http::Request) {
    println!("DEBUG REQUEST:\n{}", request);
    let response = self.handle_request(request);
    println!("DEBUG RESPONSE:\n{}", response);
  }
}

impl RequestHandler for App {
  fn handle_request(&self, request: http::Request) -> http::Response {
    let mresponder = self.routes.get(request.uri.path.as_slice());
    match mresponder {
      Some(responder) => responder.handle(request),
      None => http::Response::error404()
    }
  }
}
