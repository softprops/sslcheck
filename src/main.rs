extern crate hyper;
use hyper::Client;
use std::io;

fn main() {
  let mut client = Client::new();
    let mut res =
        client.get("https://www.howsmyssl.com/a/check").send().unwrap();
  io::copy(&mut res, &mut io::stdout()).unwrap();
}
