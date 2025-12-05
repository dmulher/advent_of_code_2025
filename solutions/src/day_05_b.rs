extern crate test;

pub fn main(contents: String) -> u128 {
  get_freshness(contents)
}

fn get_freshness(contents: String) -> u128 {
  let (ranges_s, _) = contents.split_once("\r\n\r\n").unwrap();
  get_ranges(ranges_s)
}

fn get_ranges(ranges: &str) -> u128 {
  let mut initial_ranges = ranges
    .lines()
    .map(|l| l.split_once("-").unwrap())
    .map(|(s, e)| (s.parse::<u64>().unwrap(), e.parse::<u64>().unwrap()))
    .collect::<Vec<(u64, u64)>>();
  initial_ranges.sort_by(|(a_s, _), (b_s, _)| a_s.cmp(b_s));

  let mut final_sum: u128 = 0;
  let mut current_range = None;
  for range in initial_ranges {
    match current_range {
      None => current_range = Some(range),
      Some((c_s, c_e)) => {
        if range.0 <= c_e {
          current_range = Some((c_s, c_e.max(range.1)));
        } else {
          final_sum += (c_e - c_s + 1) as u128;
          current_range = Some(range);
        }
      }
    }
  }
  if let Some((s, e)) = current_range {
    final_sum += (e - s + 1) as u128;
  }
  final_sum
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 5;
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_05_b() {
    const EXAMPLE_ANSWER: Option<u128> = Some(14);
    const ANSWER: Option<u128> = Some(343329651880509);
    match utils::run_method::<u128>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  // #[bench]
  // fn bench_day_05_b(b: &mut Bencher) {
  //   let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
  //   b.iter(|| main(input.clone()));
  // }
}
