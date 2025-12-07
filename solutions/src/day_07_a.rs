use std::collections::HashSet;

extern crate test;

pub fn main(contents: String) -> u16 {
  count_splits(contents)
}

fn count_splits(contents: String) -> u16 {
  let mut lines = contents.lines();
  let mut active_beams: Vec<bool> = lines.next().unwrap().chars().map(|c| c == 'S').collect();
  let mut splits: u16 = 0;
  contents
    .lines()
    .map(|line| line
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == '^')
        .map(|(idx, _)| idx))
    .for_each(|line| {
      for idx in line {
        if active_beams[idx] {
          active_beams[idx] = false;
          active_beams[idx-1] = true;
          active_beams[idx+1] = true;
          splits += 1;
        }
      }
    });
  splits
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 7;
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_07_a() {
    const EXAMPLE_ANSWER: Option<u16> = Some(21);
    const ANSWER: Option<u16> = Some(1560);
    match utils::run_method::<u16>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_07_a(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
