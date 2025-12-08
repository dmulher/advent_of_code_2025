extern crate test;

pub fn main(contents: String) -> u64 {
  count_splits(contents)
}

fn count_splits(contents: String) -> u64 {
  let mut lines = contents.lines();
  let mut timelines: Vec<u64> = lines.next().unwrap().chars().map(|c| if c == 'S' {1u64} else {0u64}).collect();
  contents
    .lines()
    .for_each(|line| {
      line
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == '^')
        .map(|(idx, _)| idx)
        .for_each(|split| {
          timelines[split-1] += timelines[split];
          timelines[split+1] += timelines[split];
          timelines[split] = 0;
        });
    });
  timelines.into_iter().sum()
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 7;
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_07_b() {
    const EXAMPLE_ANSWER: Option<u64> = Some(40);
    const ANSWER: Option<u64> = Some(25592971184998);
    match utils::run_method::<u64>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_07_b(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
