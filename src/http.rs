use uri;
use std::collections::{HashMap};
use std::str::from_str;

#[deriving(Show,Clone)]
pub enum Status {
  Ok,
  NotFound
}

#[deriving(Show,Clone)]
pub enum Method {
  GET,
  POST,
  DELETE,
  PATCH,
  PUT,
  HEAD,
  OPTIONS,
  // TODO: more
}

#[deriving(Show,Clone)]
pub struct Request {
  pub method: Method,
  pub uri: uri::Uri
}

impl Request {
  pub fn new(method: Method, uri: &str) -> Request {
    Request {
      method: method,
      uri: from_str::<uri::Uri>(uri).unwrap()
    }
  }
}

#[deriving(Show)]
pub struct Response {
  pub body: String,
  pub headers: HashMap<String, String>,
  pub status: Status
}

impl Response {
  pub fn from_request(req: Request) -> Response {
    Response { body: "".to_string(), headers: HashMap::new(), status: Status::Ok }
  }

  pub fn error404() -> Response {
    Response {
      body: "Not Found".to_string(),
      status: Status::NotFound,
      headers: HashMap::new(),
    }
  }
}

