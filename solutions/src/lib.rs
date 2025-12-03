#![feature(test)]
#![feature(iter_array_chunks)]

mod day_01_a;
mod day_01_b;
mod day_02_a;
mod day_02_b;
mod day_03_a;
mod day_03_b;


pub fn run_all_days() {
    day_01_a::main("".to_string());
    day_01_b::main("".to_string());
    day_02_a::main("".to_string());
    day_02_b::main("".to_string());
    day_03_a::main("".to_string());
    day_03_b::main("".to_string());
}