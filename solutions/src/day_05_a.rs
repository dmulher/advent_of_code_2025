extern crate test;

pub fn main(contents: String) -> u64 {
  get_freshness(contents)
}

fn get_freshness(contents: String) -> u64 {
  let (ranges_s, ids) = contents.split_once("\r\n\r\n").unwrap();
  let ranges = get_ranges(ranges_s);
  let mut fresh: u64 = 0;
  for id in ids.lines().map(|id| id.parse::<u64>().unwrap()) {
    for (start, end) in ranges.iter() {
      if start <= &id && end >= &id {
        fresh += 1;
        break;
      }
    }
  }
  fresh
}

fn get_ranges(ranges: &str) -> Vec<(u64, u64)> {
  ranges
    .lines()
    .map(|l| l.split_once("-").unwrap())
    .map(|(s, e)| (s.parse::<u64>().unwrap(), e.parse::<u64>().unwrap()))
    .collect::<Vec<(u64, u64)>>()
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 5;
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_05_a() {
    const EXAMPLE_ANSWER: Option<u64> = Some(3);
    const ANSWER: Option<u64> = Some(789);
    match utils::run_method::<u64>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_05_a(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
