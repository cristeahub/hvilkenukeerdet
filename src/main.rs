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
                    .send().unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    let parse: Vec<&str> = body
                            .rsplitn(2, "<body>")
                            .collect();

    let week_number: Vec<&str> = parse[0].matches(char::is_numeric).collect();

    if week_number.len() > 1 {
        println!("{}{}", week_number[0], week_number[1]);
    } else {
        println!("{}", week_number[0]);
    }
}
