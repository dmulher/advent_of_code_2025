#![feature(test)]
#![feature(iter_array_chunks)]

mod day_01_a;
mod day_01_b;


pub fn run_all_days() {
    day_01_a::main("".to_string());
    day_01_b::main("".to_string());
}