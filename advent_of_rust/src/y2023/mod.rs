mod day_1;
mod day_2;

pub fn get_solutions() -> Vec<Box<dyn crate::IsCorrect>> {
    vec![
        Box::new(day_1::get_solution()),
        Box::new(day_2::get_solution()),
    ]
}
