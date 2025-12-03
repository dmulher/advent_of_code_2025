extern crate test;

pub fn main(contents: String) -> u32 {
  get_joltage(contents)
}

fn get_joltage(contents: String) -> u32 {
  contents
    .lines()
    .map(get_max_joltage)
    .sum()
}

fn get_max_joltage(batteries: &str) -> u32 {
  let largest_batteries = batteries
    .chars()
    .map(|c: char| c.to_digit(10).unwrap())
    .fold((0u32, 0u32), |acc, b| {
      if acc.1 > acc.0 {
        (acc.1, b)
      } else if b > acc.1 {
        (acc.0, b)
      } else {
        acc
      }
    });
  largest_batteries.0 * 10 + largest_batteries.1
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 3;
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_03_a() {
    const EXAMPLE_ANSWER: Option<u32> = Some(357);
    const ANSWER: Option<u32> = Some(17031);
    match utils::run_method::<u32>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_03_a(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
