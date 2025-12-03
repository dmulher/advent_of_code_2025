extern crate test;

pub fn main(contents: String) -> u64 {
  get_joltage(contents)
}

fn get_joltage(contents: String) -> u64 {
  contents
    .lines()
    .map(get_max_joltage)
    .sum()
}

fn get_max_joltage(batteries: &str) -> u64 {
  batteries
    .chars()
    .map(|c: char| c.to_digit(10).unwrap())
    .fold([0u64; 13], |mut acc, b| {
      let mut reducing = false;
      acc[12] = b as u64;
      for i in 0..12 {
        if reducing || acc[i] < acc[i+1] {
          acc[i] = acc[i+1];
          reducing = true;
        }
      }
      acc
    })
    .into_iter()
    .take(12)
    .reduce(|acc, x| acc * 10 + x)
    .unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 3;
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_03_b() {
    const EXAMPLE_ANSWER: Option<u64> = Some(3121910778619);
    const ANSWER: Option<u64> = None;
    match utils::run_method::<u64>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_03_b(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
