extern crate emval;
use emval::*;

fn main() {
  let window = JSObj::global("window");
  let name: String = window.call_prop("prompt", args!("What is your name?"));
  window.call_prop::<()>("alert", args!(format!("Hi, {} from rust!", name)));
}
