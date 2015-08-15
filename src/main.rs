extern crate hyper;

use std::io::Read;
use hyper::Client;
use hyper::header::Connection;

static WEEKDAY_WEBSITE: &'static str = "http://www.hvilkenukeerdet.no";

fn main() {
    let client = Client::new();

    let mut res = client
                    .get(WEEKDAY_WEBSITE)
                    .header(Connection::close())
                    .send()
                    .unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    let body_index = body.find("<body>\n").unwrap() + "<body>\n".len();
    let break_index = body.find("<br />").unwrap();

    let week_number = &body[body_index..break_index];
    println!("{}", week_number);
}
