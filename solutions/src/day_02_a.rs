extern crate test;

pub fn main(contents: String) -> u64 {
  get_invalid_ids(contents)
}

fn get_invalid_ids(contents: String) -> u64 {
  contents
    .split(",")
    .flat_map(get_range)
    .map(|(first, last, l)| get_invalid_ids_in_range(first, last, l))
    .sum()
}

fn get_range(range: &str) -> Vec<(u64, u64, u8)> {
  let (start, end) = range.split_once("-").unwrap();
  let first_is_valid = start.len() % 2 == 0;
  let last_is_valid = end.len() % 2 == 0;
  let first_len = if first_is_valid { start.len() } else { start.len() + 1 };
  let last_len = if last_is_valid { end.len() } else { end.len() - 1 };
  let first = if first_is_valid { start.parse::<u64>().unwrap() } else { 10u64.pow(start.len() as u32) };
  let last = if last_is_valid { end.parse::<u64>().unwrap() } else { 10u64.pow(end.len() as u32 - 1) - 1 };
  if first > last {
    vec![]
  } else {
    (first_len..=last_len)
      .filter(|l| l % 2 == 0)
      .map(|l| {
        (first.max(10u64.pow(l as u32 - 1)), last.min(10u64.pow(l as u32) - 1), (l / 2) as u8)
      })
      .collect()
  }
}

fn get_invalid_ids_in_range(start: u64, end: u64, mid_point: u8) -> u64 {
  let div =  10u64.pow(mid_point as u32);
  ((start / div)..=(end / div))
    .map(|candidate| (candidate * div) + candidate)
    .filter(|candidate| candidate >= &start && candidate <= &end)
    .sum()
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 2;
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_02_a() {
    const EXAMPLE_ANSWER: Option<u64> = Some(1227775554);
    const ANSWER: Option<u64> = Some(38310256125);
    match utils::run_method::<u64>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_02_a(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
