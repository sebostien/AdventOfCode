use std::path::Path;

use anyhow::anyhow;
use reqwest::header::COOKIE;

pub fn get_input_contents(year: u32, day: u32) -> anyhow::Result<String> {
    let input_file_name = format!("../input/Y{year}/Day{day}.txt");

    if let Err(e) = std::fs::create_dir_all(format!("../input/Y{year}")) {
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
        match load_session_cookie() {
            None => Err(anyhow!("Could not find AOC cookie!")),
            Some(cookie_string) => {
                let resp = donwload_file(
                    &format!("https://adventofcode.com/{year}/day/{day}/input"),
                    &input_file_name,
                    &cookie_string,
                );

                match resp {
                    Ok(text) => Ok(text),
                    Err(err) => {
                        println!("Error downloading file:\n{err}");
                        Err(anyhow!("Could not download input file for {year}/{day}"))
                    }
                }
            }
        }
    }
}

#[must_use]
fn load_session_cookie() -> Option<String> {
    match dotenv::from_filename("../.env") {
        Ok(_) => match std::env::var("SESSION_COOKIE") {
            Ok(s) => Some(s),
            Err(_) => None,
        },
        Err(_) => None,
    }
}

fn donwload_file(
    url: &str,
    file: &str,
    cookie: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    println!("Downloading file: {url}");

    let client = reqwest::blocking::Client::new();

    let resp = client
        .get(url)
        .header(COOKIE, format!("session={cookie}"))
        .send()?
        .text()?;

    std::fs::write(file, &resp)?;
    Ok(resp)
}

#[must_use]
pub fn get_year_day(file: &str) -> (u32, u32) {
    let split = file.split('/').skip(1).collect::<Vec<&str>>();

    if let [year, day] = split[..] {
        (
            year[1..].parse().unwrap(),
            day[4..].strip_suffix(".rs").unwrap().parse().unwrap(),
        )
    } else {
        panic!("File is not named correctly: {file}");
    }
}
