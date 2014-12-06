use http;

pub trait Responder: Sync + Send {
  fn handle(&self, req: http::Request) -> http::Response;
}

pub trait ToResponder {
  fn to_responder<'a>(&self) -> Box<Responder + 'a>;
}

impl Responder for fn(http::Request) -> http::Response {
  fn handle(&self, req: http::Request) -> http::Response {
    (*self)(req)
  }
}

impl ToResponder for fn(http::Request) -> http::Response {
  fn to_responder<'a>(&self) -> Box<Responder + 'a> {
    box self.clone()
  }
}
