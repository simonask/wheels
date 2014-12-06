#![feature(default_type_params)]

extern crate hyper;

pub use app::App;
pub use http::{Request, Response, Status};
pub use uri::Uri;
pub use routes::Responder;

pub mod app;
pub mod http;
pub mod uri;
pub mod routes;
mod server;

pub fn render_text<T: std::str::StrAllocating>(s: T) -> Response {
  Response {
    status: Status::Ok,
    headers: std::collections::HashMap::new(),
    body: s.into_string()
  }
}


#[test]
fn it_works() {
}
