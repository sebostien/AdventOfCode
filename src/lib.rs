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

pub mod _include;
pub mod download;

mod util;
mod y2021;
mod y2022;
mod y2023;
mod y2024;

type SolFn<T> = fn(&str) -> anyhow::Result<T>;

pub struct Solution<A: Eq + Display, B: Eq + Display> {
    pub part_1: SolFn<A>,
    pub part_2: SolFn<B>,
    pub answer: (A, B),
}

pub trait IsCorrect {
    fn part_1(&self, input: &str) -> anyhow::Result<(bool, String)>;
    fn part_2(&self, input: &str) -> anyhow::Result<(bool, String)>;
}

impl<A: Eq + Display, B: Eq + Display> IsCorrect for Solution<A, B> {
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

#[cfg(test)]
mod tests {
    use crate::{download::get_input_contents, IsCorrect};

    /// Test `year`, `day` and `part`. Panics with messages if incorrect.
    pub fn test_year_day(year: usize, day: usize, part: usize, solution: &dyn IsCorrect) {
        let input = match get_input_contents(year, day) {
            Ok(input) => input,
            Err(err) => panic!("{err}"),
        };

        if part == 1 {
            match solution.part_1(&input) {
                Err(err) => {
                    panic!("Error when running {year}/{day} :: Part 1\n{err}");
                }
                Ok((correct, msg)) => {
                    assert!(
                        correct,
                        "---- Incorrect solution for {year}/{day} :: Part 1 ----\n{msg}"
                    );
                }
            }
        } else {
            match solution.part_2(&input) {
                Err(err) => {
                    println!("Error when running {year}/{day} :: Part 2\n{err}");
                }
                Ok((correct, msg)) => {
                    assert!(
                        correct,
                        "---- Incorrect solution for {year}/{day} :: Part 2 ----\n{msg}"
                    );
                }
            }
        }
    }
}
