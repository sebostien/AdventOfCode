#![warn(
    missing_copy_implementations,
    clippy::all,
    clippy::doc_markdown,
    clippy::dbg_macro,
    clippy::todo,
    clippy::empty_enum,
    clippy::enum_glob_use,
    clippy::mem_forget,
    clippy::use_self,
    clippy::filter_map_next,
    clippy::needless_continue,
    clippy::needless_borrow,
    clippy::match_wildcard_for_single_variants,
    clippy::if_let_mutex,
    clippy::mismatched_target_os,
    clippy::await_holding_lock,
    clippy::match_on_vec_items,
    clippy::imprecise_flops,
    clippy::lossy_float_literal,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::fn_params_excessive_bools,
    clippy::exit,
    clippy::inefficient_to_string,
    clippy::linkedlist,
    clippy::macro_use_imports,
    clippy::option_option,
    clippy::verbose_file_reads,
    clippy::unnested_or_patterns,
    clippy::absurd_extreme_comparisons,
    rust_2018_idioms,
    future_incompatible,
    nonstandard_style,
    clippy::correctness,
    clippy::suspicious,
    clippy::complexity,
    clippy::perf,
    clippy::style,
    clippy::pedantic
)]
#![allow(
    clippy::unnecessary_wraps,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap
)]

use std::fmt::Display;

use util::get_input_contents;

pub mod util;
mod y2021;
mod y2022;
mod y2023;

type SolFn<T> = Box<dyn Fn(&str) -> anyhow::Result<T>>;

pub struct Solution<A: Eq + Display, B: Eq + Display> {
    pub date: (u32, u32),
    pub part_1: SolFn<A>,
    pub part_2: SolFn<B>,
    pub answer: (A, B),
}

pub trait IsCorrect {
    fn get_date(&self) -> (u32, u32);
    fn part_1(&self, input: &str) -> anyhow::Result<(bool, String)>;
    fn part_2(&self, input: &str) -> anyhow::Result<(bool, String)>;
}

impl<A: Eq + Display, B: Eq + Display> IsCorrect for Solution<A, B> {
    fn get_date(&self) -> (u32, u32) {
        self.date
    }

    fn part_1(&self, input: &str) -> anyhow::Result<(bool, String)> {
        let ans = (self.part_1)(input)?;
        Ok((
            ans == self.answer.0,
            format!("Answer:\n{ans}\nCorrect:\n{}", self.answer.0),
        ))
    }

    fn part_2(&self, input: &str) -> anyhow::Result<(bool, String)> {
        let ans = (self.part_2)(input)?;
        Ok((
            ans == self.answer.1,
            format!("Answer:\n{ans}\nCorrect:\n{}", self.answer.1),
        ))
    }
}

#[must_use]
pub fn get_all_years() -> Vec<(u32, Vec<Box<dyn IsCorrect>>)> {
    vec![
        (2022, crate::y2022::get_solutions()),
        (2023, crate::y2023::get_solutions()),
    ]
}

#[must_use]
pub fn test_year_day(year: u32, day: u32, solution: &dyn IsCorrect) -> (bool, String) {
    let input = match get_input_contents(year, day) {
        Ok(input) => input,
        Err(err) => return (false, err.to_string()),
    };

    let mut is_correct = true;
    match solution.part_1(&input) {
        Err(err) => {
            is_correct = false;
            println!("Error when running {year}/{day} :: Part 1\n{err}");
        }
        Ok(correct) => {
            if !correct.0 {
                is_correct = false;
                println!(
                    "\n---- Incorrect solution for {year}/{day} :: Part 1 ----\n{}",
                    correct.1
                );
            }
        }
    }
    match solution.part_2(&input) {
        Err(err) => {
            is_correct = false;
            println!("Error when running {year}/{day} :: Part 2\n{err}");
        }
        Ok(correct) => {
            if !correct.0 {
                is_correct = false;
                println!(
                    "{}\n---- Incorrect solution for {year}/{day} :: Part 2 ----\n{}\n{}",
                    "-".repeat(50),
                    correct.1,
                    "-".repeat(50)
                );
            }
        }
    }
    (is_correct, format!("{year}/{day}"))
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
                let (correct, date) = test_year_day(year, day, sol.as_ref());
                assert!(correct);
                tests.push(date);
            }

            println!("-- {y1} ------------------");
            println!("{}", tests.join("\n"));
        }
    }
}
