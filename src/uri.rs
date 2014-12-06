use std::string::{String};
use std::str::FromStr;

#[deriving(Show,Clone)]
pub struct Uri {
  pub path: String
}

impl FromStr for Uri {
  fn from_str(input: &str) -> Option<Uri> {
    Some(Uri { path: input.to_string() })
  }
}
