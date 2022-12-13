mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod day_10;
mod day_11;
mod day_12;
mod day_13;

pub fn get_solutions() -> Vec<Box<dyn crate::IsCorrect>> {
    vec![
        Box::new(day_1::get_solution()),
        Box::new(day_2::get_solution()),
        Box::new(day_3::get_solution()),
        Box::new(day_4::get_solution()),
        Box::new(day_5::get_solution()),
        Box::new(day_6::get_solution()),
        Box::new(day_7::get_solution()),
        Box::new(day_8::get_solution()),
        Box::new(day_9::get_solution()),
        Box::new(day_10::get_solution()),
        Box::new(day_11::get_solution()),
        Box::new(day_12::get_solution()),
        Box::new(day_13::get_solution()),
    ]
}


