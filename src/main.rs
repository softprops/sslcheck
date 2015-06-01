extern crate hyper;
use hyper::Client;
use std::io;

fn main() {
  let mut client = Client::new();
  // thread '<main>' panicked at 'called `Result::unwrap()` on an `Err` value: Ssl(OpenSslErrors([UnknownError { library: "SSL routines", function: "SSL23_GET_SERVER_HELLO", reason: "tlsv1 alert protocol version" }]))'
  let mut res =
        client.get("https://www.howsmyssl.com/a/check").send().unwrap();
  io::copy(&mut res, &mut io::stdout()).unwrap();
}
