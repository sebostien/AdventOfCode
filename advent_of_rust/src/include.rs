// This file was generated by build.rs.
// Do not modify directly

#[must_use]
pub fn get_all_years() -> Vec<(u32, Vec<Box<dyn crate::IsCorrect>>)> {
    vec![
        (2021, crate::y2021::get_solutions()),
        (2022, crate::y2022::get_solutions()),
        (2023, crate::y2023::get_solutions())
    ]
}
