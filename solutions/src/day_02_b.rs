use std::collections::HashSet;

extern crate test;

pub fn main(contents: String) -> u64 {
  get_invalid_ids(contents)
}

fn get_invalid_ids(contents: String) -> u64 {
  contents
    .split(",")
    .flat_map(get_ranges)
    .map(|(first, last, l)| get_invalid_ids_in_range(first, last, l))
    .sum()
}

fn get_ranges(range: &str) -> Vec<(u64, u64, u8)> {
  let (start, end) = range.split_once("-").unwrap();
  let first = start.parse::<u64>().unwrap();
  let last = end.parse::<u64>().unwrap();
  (start.len()..=end.len())
    .map(|l| {
      (first.max(10u64.pow(l as u32 - 1)), last.min(10u64.pow(l as u32) - 1), l as u8)
    })
    .collect()
}

fn get_invalid_ids_in_range(start: u64, end: u64, length: u8) -> u64 {
  (1..=(length/2))
    .filter(|l| length % l == 0)
    .flat_map(|l| {
      let div = 10u64.pow(l as u32);
      let factor = (length / l) as u32;
      ((start / div.pow(factor - 1))..=(end / div.pow(factor - 1)))
        .map(|candidate| (0..factor).fold(0, |acc, _| (acc * div) + candidate))
        .filter(|candidate| candidate >= &start && candidate <= &end)
        .collect::<Vec<u64>>()
    })
    .collect::<HashSet<u64>>()
    .into_iter()
    .sum()
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 2;
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_02_b() {
    const EXAMPLE_ANSWER: Option<u64> = Some(4174379265);
    const ANSWER: Option<u64> = Some(58961152806);
    match utils::run_method::<u64>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_02_b(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
