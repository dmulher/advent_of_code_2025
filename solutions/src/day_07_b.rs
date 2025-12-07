use std::collections::HashMap;

extern crate test;

pub fn main(contents: String) -> u64 {
  count_splits(contents)
}

fn count_splits(contents: String) -> u64 {
  contents
    .lines()
    .fold(HashMap::<usize, u64>::new(), |mut acc, line| {
      if acc.keys().len() == 0 {
        HashMap::<usize, u64>::from([(line
          .chars()
          .enumerate()
          .filter(|(_, c)| *c == 'S')
          .next()
          .unwrap()
          .0, 1)])
      } else {
        let timelines = acc.clone();
        line
          .chars()
          .enumerate()
          .filter(|(idx, c)| *c == '^' && timelines.contains_key(idx))
          .map(|(idx, _)| idx)
          .for_each(|split| {
            let timelines = acc.remove(&split).unwrap();
            *acc.entry(split-1).or_insert(0) += timelines;
            *acc.entry(split+1).or_insert(0) += timelines;
          });
        acc
      }
    })
    .into_iter()
    .fold(0, |acc, (_, t)| acc + t)
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
    const EXAMPLE_ANSWER: Option<u64> = Some(40);
    const ANSWER: Option<u64> = Some(25592971184998);
    match utils::run_method::<u64>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
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
