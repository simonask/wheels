Wheels Web Framework
====================

Wheels is a framework for HTTP apps developed in Rust.

It is still in very early development (like Rust itself).


By Example
----------

Hello World in Wheels:

```
extern crate "wheels" as w;

fn hello(req: w::Request) -> w::Response {
  w::render_text("Hello, World!")
}

fn main() {
  let mut app = w::App::new();
  app.get("/", &hello);
  app.listen("0.0.0.0", 3000);
}
```
