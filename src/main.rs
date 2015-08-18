extern crate hyper;
extern crate chrono;

use std::env;
use std::io::Read;
use hyper::Client;
use hyper::header::Connection;

static WEEKDAY_WEBSITE: &'static str = "http://www.hvilkenukeerdet.no";

fn print_help() {
    println!("`uke` gives the current week number.
             `uke <number>` gives you the range for that week number");
}

fn parse_string_to_int(s: String) -> usize {
    let mut res: usize = 0;
    let bytes = s.into_bytes();
    let b_len = bytes.len();
    for i in 0..b_len {
        // 48 is the byte number for the string "0"
        let natural_number = (bytes[(b_len-1) - i] as usize) - 48;
        if i > 0 {
            res += i * 10 * natural_number
        } else {
            res += natural_number;
        }
    }

    if res > 52 {
        return 0;
    } else {
        return res;
    }
}

fn show_week_period_for_week_number(w: usize) {
    // TODO: implement the actual week number checker
}

fn main() {
    if let Some(arg1) = env::args().nth(1) {
        let week_number = parse_string_to_int(arg1);
        if week_number == 0 {
            print_help();
        } else {
            show_week_period_for_week_number(week_number);
        }
    } else {
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
}
