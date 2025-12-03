use chrono::{Datelike, TimeZone};
use std::path::Path;

use anyhow::anyhow;
use reqwest::header::COOKIE;

pub fn get_input_contents(year: usize, day: usize) -> anyhow::Result<String> {
    let input_file_name = format!("./input/Y{year}/Day{day}.txt");

    if let Err(e) = std::fs::create_dir_all(format!("./input/Y{year}")) {
        match e.kind() {
            std::io::ErrorKind::AlreadyExists => {}
            _ => {
                return Err(anyhow!("Could not create directory for input!"));
            }
        }
    }

    if Path::new(input_file_name.as_str()).exists() {
        let result = std::fs::read_to_string(input_file_name);
        match result {
            Ok(text) => Ok(text),
            Err(_) => Err(anyhow!("Could not read local input file!")),
        }
    } else {
        let date = &chrono_tz::US::Eastern.from_utc_datetime(&chrono::Utc::now().naive_utc());
        let c_year = date.year() as usize;
        let c_month = date.month() as usize;
        let c_day = date.day() as usize;

        if year > c_year || year == c_year && (c_month < 12 || c_day < day) {
            return Err(anyhow!("Day is not released yet {year}/{day}"));
        }

        match load_session_cookie() {
            None => Err(anyhow!("Could not find AOC cookie!")),
            Some(cookie_string) => {
                eprintln!("WARNING! Downloading input file. Please make sure this is not ran multiple times!!!");
                let resp = download_file(
                    &format!("https://adventofcode.com/{year}/day/{day}/input"),
                    &input_file_name,
                    &cookie_string,
                );

                match resp {
                    Ok(text) => Ok(text),
                    Err(err) => Err(anyhow!(
                        "Could not download input file for {year}/{day}: {err}"
                    )),
                }
            }
        }
    }
}

#[must_use]
fn load_session_cookie() -> Option<String> {
    match dotenv::from_filename("./.env") {
        Ok(_) => match std::env::var("SESSION_COOKIE") {
            Ok(s) => Some(s),
            Err(_) => None,
        },
        Err(_) => None,
    }
}

fn download_file(
    url: &str,
    file: &str,
    cookie: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();

    let resp = client
        .get(url)
        .header(COOKIE, format!("session={cookie}"))
        .send()?
        .text()?;

    std::fs::write(file, &resp)?;
    Ok(resp)
}
