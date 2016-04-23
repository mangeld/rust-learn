extern crate hyper;

use std::io::Read;

use hyper::Client;
use hyper::header::Connection;

fn main() {
    let client = Client::new();
    let mut res = client.get("https://mangel.me")
        .header(Connection::close())
        .send().unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    println!("response: {}", body)
} 
