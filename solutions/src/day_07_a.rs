use std::collections::HashSet;

extern crate test;

pub fn main(contents: String) -> u16 {
  count_splits(contents)
}

fn count_splits(contents: String) -> u16 {
  contents
    .lines()
    .fold((0u16, HashSet::<usize>::new()), |acc, line| {
      if acc.1.len() == 0 {
        (0, HashSet::from([line
          .chars()
          .enumerate()
          .filter(|(_, c)| *c == 'S')
          .next()
          .unwrap()
          .0]))
      } else {
        let beams = acc.1.clone();
        let splits = line
          .chars()
          .enumerate()
          .filter(|(idx, c)| *c == '^' && beams.contains(idx))
          .map(|(idx, _)| idx);
        let split_idxs = splits.clone().collect::<Vec<usize>>();
        (
          acc.0 + splits.clone().count() as u16,
          acc.1
            .into_iter()
            .filter(|beam| !split_idxs.contains(beam))
            .chain(splits.flat_map(|split| [split-1, split+1]))
            .collect::<HashSet<usize>>()
        )
      }
    })
    .0
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
