extern crate hyper;
extern crate chrono;

use std::env;
use std::io::Read;
use chrono::*;
use hyper::Client;
use hyper::header::Connection;

static WEEKDAY_WEBSITE: &'static str = "http://www.hvilkenukeerdet.no";

fn print_help() {
    println!("`uke` gives the current week number.
             `uke <number>` gives you the range for that week number");
}

fn is_leap_year(year: i32) -> bool {
    return (year % 4 == 0) && (year % 100 != 0) || (year % 400 == 0)
}

fn find_num_of_weeks_this_year(year: i32) -> usize {
    let last_day_of_year = UTC.ymd(year, 12, 31);
    if last_day_of_year.weekday() == Weekday::Thu ||
        (last_day_of_year.weekday() == Weekday::Fri &&
         is_leap_year(year)) {
        return 53;
    } else {
        return 52;
    }
}

fn show_week_period_for_week_number(w: usize, year: i32) {
    let mut start_date = UTC.ymd(year, 1, 1).and_hms(1,0,0);
    let mut current_week = 1;

    // week 1 is defined by the first week with a thursday
    // in a year

    let jan_1_week_day = start_date.weekday().number_from_monday();
    if jan_1_week_day > 4 {
        /* this week has no thursday */
        /* the calculation will give the date of monday in the first week*/
        start_date = UTC.ymd(year, 1, (1 + 8-jan_1_week_day)).and_hms(1,0,0);
    } else {
        /* start at this week's first day */
        start_date = UTC.ymd(year-1, 12, (31 - (jan_1_week_day - 2))).and_hms(1,0,0);
    }

    while w != current_week {
        start_date = start_date + Duration::weeks(1);
        current_week += 1;
    }

    let end_date = start_date + Duration::days(6);

    println!("Week {}, {} is from \"{}\" to \"{}\"",
             w,
             year,
             start_date.format("%d %b"),
             end_date.format("%d %b")
            );
}

fn main() {
    let year = Local::now().year();

    if let Some(arg1) = env::args().nth(1) {
        let num_of_weeks_this_year = find_num_of_weeks_this_year(year);
        match arg1.parse::<usize>() {
            Ok(week_number) =>
                if week_number >= 1 && week_number <= num_of_weeks_this_year {
                    show_week_period_for_week_number(week_number, year);
                    return;
                },
            _ => {} ,
        }

        print_help();
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
