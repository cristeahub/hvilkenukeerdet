extern crate chrono;

use chrono::*;
use std::env;


fn print_help() {
    println!("`uke` gives the current week number.
              `uke <number>` gives you the range for that week number");
}

fn is_leap_year(year: i32) -> bool {
    return (year % 4 == 0) && (year % 100 != 0) || (year % 400 == 0)
}

fn find_num_of_weeks_this_year(year: i32) -> usize {
    let last_day_of_year = UTC.ymd(year, 12, 31);
    return match last_day_of_year.weekday() {
        Weekday::Thu | Weekday::Fri if is_leap_year(year) => 53,
        _ => 52,
    };
}

fn show_week_period_for_week_number(w: usize, year: i32) {
    let mut start_date = UTC.ymd(year, 1, 1);

    /* week 1 is defined as the first week with a thursday in a year */

    let jan_1_week_day = start_date.weekday().number_from_monday();
    if jan_1_week_day > 4 {
        /* this week has no thursday */
        /* the calculation will give the date of monday in the first week */
        start_date = UTC.ymd(year, 1, (1 + 8-jan_1_week_day));
    } else {
        /* start at this week's first day */
        start_date = UTC.ymd(year-1, 12, (31 - (jan_1_week_day - 2)));
    }

    start_date = start_date + Duration::weeks((w as i64) - 1);

    let end_date = start_date + Duration::days(6);

    println!("Week {}, {} is from \"{}\" to \"{}\"",
             w,
             year,
             start_date.format("%d %b"),
             end_date.format("%d %b")
            );
}

fn get_week_number() -> isize {
    return 42; //guaranteed to work next week
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
            _ => {},
        }

        print_help();
    } else {
        println!("{}", get_week_number());
    }
}
