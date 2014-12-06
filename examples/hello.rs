extern crate "wheels" as w;

fn hello(req: w::Request) -> w::Response {
  w::render_text("Hello, World!")
}

fn main() {
  let mut app = w::App::new();
  app.get("/", &hello);
  app.listen("0.0.0.0", 3000);
}
