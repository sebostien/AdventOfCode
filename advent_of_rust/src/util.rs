use std::path::Path;

use reqwest::header::COOKIE;

#[derive(Debug, Clone, Copy)]
pub enum GetInputContentsError {
    CouldNotDownload,
    CouldNotReadLocal,
    CookieNotFound,
    CouldNotCreateDir,
}

pub fn get_input_contents(year: u32, day: u32) -> Result<String, GetInputContentsError> {
    use GetInputContentsError::*;

    let input_file_name = format!("../input/Y{}/Day{}.txt", year, day);

    if let Err(e) = std::fs::create_dir_all(format!("../input/Y{}", year)) {
        match e.kind() {
            std::io::ErrorKind::AlreadyExists => (),
            _ => return Err(CouldNotCreateDir),
        }
    }

    if Path::new(input_file_name.as_str()).exists() {
        let result = std::fs::read_to_string(input_file_name);
        match result {
            Ok(text) => Ok(text),
            Err(_) => Err(CouldNotReadLocal),
        }
    } else {
        match load_session_cookie() {
            None => Err(CookieNotFound),
            Some(cookie_string) => {
                let resp = donwload_file(
                    format!("https://adventofcode.com/{year}/day/{day}/input",),
                    input_file_name,
                    cookie_string,
                );

                match resp {
                    Ok(text) => Ok(text),
                    Err(_) => {
                        println!("Error downloading file:\n{:?}", resp);
                        Err(CouldNotDownload)
                    }
                }
            }
        }
    }
}

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
    url: String,
    file: String,
    cookie: String,
) -> Result<String, Box<dyn std::error::Error>> {
    println!("Downloading file: {}", url);

    let client = reqwest::blocking::Client::new();

    let resp = client
        .get(url)
        .header(COOKIE, format!("session={}", cookie))
        .send()?
        .text()?;

    std::fs::write(file, &resp)?;
    Ok(resp)
}

pub fn get_year_day(file: &str) -> (u32, u32) {
    let split = file.split('/').skip(1).collect::<Vec<&str>>();

    if let [year, day] = split[..] {
        (
            year[1..].parse().unwrap(),
            day[4..].strip_suffix(".rs").unwrap().parse().unwrap(),
        )
    } else {
        panic!("File is not named correctly: {}", file);
    }
}
