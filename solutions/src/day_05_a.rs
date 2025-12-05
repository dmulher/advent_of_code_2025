extern crate test;

pub fn main(contents: String) -> u64 {
  get_freshness(contents)
}

fn get_freshness(contents: String) -> u64 {
  let (ranges_s, ids) = contents.split_once("\r\n\r\n").unwrap();
  let ranges = get_ranges(ranges_s);
  let mut fresh: u64 = 0;
  ids
    .lines()
    .for_each(|id| {
      let id = id.parse::<u64>().unwrap();
      if id_in_range(&ranges, id) {
        fresh += 1;
      }
    });
  fresh
}

fn get_ranges(ranges: &str) -> Vec<(u64, u64)> {
  let mut initial_ranges = ranges
    .lines()
    .map(|l| l.split_once("-").unwrap())
    .map(|(s, e)| (s.parse::<u64>().unwrap(), e.parse::<u64>().unwrap()))
    .collect::<Vec<(u64, u64)>>();
  initial_ranges.sort_by(|(a_s, _), (b_s, _)| a_s.cmp(b_s));

  let mut final_ranges: Vec<(u64, u64)> = Vec::new();
  let mut current_range = None;
  for range in initial_ranges {
    match current_range {
      None => current_range = Some(range),
      Some((c_s, c_e)) => {
        if range.0 < c_e {
          current_range = Some((c_s, c_e.max(range.1)));
        } else {
          final_ranges.push((c_s, c_e));
          current_range = Some(range);
        }
      }
    }
  }
  final_ranges.push(current_range.unwrap());
  final_ranges
}

fn id_in_range(ranges: &Vec<(u64, u64)>, id: u64) -> bool {
  let (mut low, mut high) = (0, ranges.len() - 1);
  while high >= low {
    let idx = (high + low) / 2;
    let (s, e) = ranges[idx];
    if id < s {
      if idx == 0 {
        return false;
      }
      high = idx - 1;
    } else if id > e {
      if idx == ranges.len() - 1 {
        return false;
      }
      low = idx + 1;
    } else {
      return true;
    }
  }
  false
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
