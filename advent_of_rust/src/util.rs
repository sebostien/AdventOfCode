use std::path::Path;
use std::sync::Once;

use reqwest::header::COOKIE;

static ENV_INIT: Once = Once::new();

#[derive(Debug)]
pub enum GetInputContentsError {
    CouldNotDownload,
    CouldNotReadLocal,
    CookieNotFound,
    CouldNotCreateDir,
}

pub fn get_input_contents(year: u32, day: u32) -> Result<String, GetInputContentsError> {
    use GetInputContentsError::*;

    let input_file_name = format!("../input/Y{}/Day{}.txt", year, day);

    if let Err(e) = std::fs::create_dir_all(&format!("../input/Y{}", year)) {
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
                    format!("https://adventofcode.com/{}/day/{}/input", year, day),
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

pub fn base_2_to_10<N: From<i32>>(num: &str) -> N {
    let mut x = 0;
    for (i, c) in num.chars().rev().enumerate() {
        if c == '1' {
            x += 2_i32.pow(i as u32);
        }
    }

    x.into()
}

pub fn base_2_to_10_slice<N: From<i32>>(num: &[char]) -> N {
    let mut x = 0;
    for (i, c) in num.iter().rev().enumerate() {
        if *c == '1' {
            x += 2_i32.pow(i as u32);
        }
    }

    x.into()
}

#[test]
fn test() {
    let x = base_2_to_10::<i32>("00000001");
    assert_eq!(x, 1);
    let x = base_2_to_10::<i32>("010");
    assert_eq!(x, 2);
    let x = base_2_to_10::<i32>("011");
    assert_eq!(x, 3);
    let x = base_2_to_10::<i32>("100");
    assert_eq!(x, 4);
    let x = base_2_to_10::<i32>("111");
    assert_eq!(x, 7);
    let x = base_2_to_10::<i32>("101100110101001010111100101011");
    assert_eq!(x, 752135979);
}
