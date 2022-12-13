use std::fmt::Display;

use util::{get_input_contents, GetInputContentsError};

pub mod util;
mod y2021;
mod y2022;

type SolFn<T> = Box<dyn Fn(&str) -> Result<T, String>>;

pub struct Solution<A: Eq + Display, B: Eq + Display> {
    pub date: (u32, u32),
    pub part_1: SolFn<A>,
    pub part_2: SolFn<B>,
    pub answer: (A, B),
}

pub trait IsCorrect {
    fn get_date(&self) -> (u32, u32);
    fn part_1(&self, input: &str) -> Result<(bool, String), String>;
    fn part_2(&self, input: &str) -> Result<(bool, String), String>;
}

impl<A: Eq + Display, B: Eq + Display> IsCorrect for Solution<A, B> {
    fn get_date(&self) -> (u32, u32) {
        self.date
    }

    fn part_1(&self, input: &str) -> Result<(bool, String), String> {
        match (self.part_1)(input) {
            Ok(ans) => Ok((
                ans == self.answer.0,
                format!("Answer:\n{}\nCorrect:\n{}", ans, self.answer.0),
            )),
            Err(err) => Err(err),
        }
    }

    fn part_2(&self, input: &str) -> Result<(bool, String), String> {
        match (self.part_2)(input) {
            Ok(ans) => Ok((
                ans == self.answer.1,
                format!("Answer:\n{}\nCorrect:\n{}", ans, self.answer.1),
            )),
            Err(err) => Err(err),
        }
    }
}

pub fn get_all_years() -> Vec<(u32, Vec<Box<dyn IsCorrect>>)> {
    vec![(2022, crate::y2022::get_solutions())]
}

pub fn test_year_day(year: u32, day: u32, solution: Box<dyn IsCorrect>) -> (bool, String) {
    let input = match get_input_contents(year, day) {
        Ok(input) => input,
        Err(err) => match err {
            GetInputContentsError::CouldNotDownload => {
                return (false, "Could not download input file!".to_string());
            }
            GetInputContentsError::CouldNotReadLocal => {
                return (false, "Could not read local input file!".to_string());
            }
            GetInputContentsError::CookieNotFound => {
                return (false, "Could not find cookie for AOC!".to_string());
            }
            GetInputContentsError::CouldNotCreateDir => {
                return (false, "Could not create directory for input!".to_string());
            }
        },
    };

    let mut is_correct = true;
    match solution.part_1(&input) {
        Err(err) => {
            is_correct = false;
            println!("Error when running {}/{} :: Part 1\n{}", year, day, err);
        }
        Ok(correct) => {
            if !correct.0 {
                is_correct = false;
                println!(
                    "\n---- Incorrect solution for {}/{} :: Part 1 ----\n{}",
                    year, day, correct.1
                );
            }
        }
    }
    match solution.part_2(&input) {
        Err(err) => {
            is_correct = false;
            println!("Error when running {}/{} :: Part 2\n{}", year, day, err);
        }
        Ok(correct) => {
            if !correct.0 {
                is_correct = false;
                println!(
                    "{}\n---- Incorrect solution for {}/{} :: Part 2 ----\n{}\n{}",
                    "-".repeat(50),
                    year,
                    day,
                    correct.1,
                    "-".repeat(50)
                );
            }
        }
    }
    (is_correct, format!("{}/{}", year, day))
}

#[cfg(test)]
mod tests {
    use crate::{get_all_years, test_year_day};

    #[test]
    fn test_all() {
        let years = get_all_years();

        for (y1, sols) in years {
            let mut tests = vec![];

            for sol in sols {
                let (year, day) = sol.get_date();
                let (correct, date) = test_year_day(year, day, sol);
                assert!(correct);
                tests.push(date);
            }

            println!("-- {y1} ------------------");
            println!("{}", tests.join("\n"));
        }
    }
}
