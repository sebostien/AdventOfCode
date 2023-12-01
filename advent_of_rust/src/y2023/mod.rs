mod day_1;

pub fn get_solutions() -> Vec<Box<dyn crate::IsCorrect>> {
    vec![
        Box::new(day_1::get_solution()),
    ]
}
