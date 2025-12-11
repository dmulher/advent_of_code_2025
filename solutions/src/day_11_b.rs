use std::collections::HashMap;

extern crate test;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Path {
  Unresolved(Vec<String>),
  Resolved(u64)
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum PathStatus {
  Trundling(String),
  Dac(String),
  Fft(String),
  Done(String),
}

pub fn main(contents: String) -> u64 {
  maze_it_out(contents)
}

fn maze_it_out(contents: String) -> u64 {
  let mut maze = contents
    .lines()
    .flat_map(|line| {
      let (start, end) = line.split_once(": ").unwrap();
      [
        (PathStatus::Trundling(start.to_string()), Path::Unresolved(end.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>())),
        (PathStatus::Dac(start.to_string()), Path::Unresolved(end.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>())),
        (PathStatus::Fft(start.to_string()), Path::Unresolved(end.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>())),
        (PathStatus::Done(start.to_string()), Path::Unresolved(end.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>()))
      ]
    })
    .collect::<HashMap<PathStatus, Path>>();
  maze.insert(PathStatus::Trundling("out".to_string()), Path::Resolved(0));
  maze.insert(PathStatus::Dac("out".to_string()), Path::Resolved(0));
  maze.insert(PathStatus::Fft("out".to_string()), Path::Resolved(0));
  maze.insert(PathStatus::Done("out".to_string()), Path::Resolved(1));
  find_end(&mut maze, PathStatus::Trundling("svr".to_string()))
}

fn find_end(map: &mut HashMap<PathStatus, Path>, pos: PathStatus) -> u64 {
  let options = map[&pos].clone();
  match options {
    Path::Resolved(existing) => existing,
    Path::Unresolved(possibilities) => {
      let result = possibilities
        .iter()
        .map(|p| {
          let next_pos = match (p.as_str(), &pos) {
            (_, PathStatus::Done(_)) => PathStatus::Done(p.clone()),
            ("dac", PathStatus::Fft(_)) => PathStatus::Done(p.clone()),
            ("fft", PathStatus::Dac(_)) => PathStatus::Done(p.clone()),
            ("dac", PathStatus::Trundling(_)) => PathStatus::Dac(p.clone()),
            ("fft", PathStatus::Trundling(_)) => PathStatus::Fft(p.clone()),
            (_, PathStatus::Fft(_)) => PathStatus::Fft(p.clone()),
            (_, PathStatus::Dac(_)) => PathStatus::Dac(p.clone()),
            (_, PathStatus::Trundling(_)) => PathStatus::Trundling(p.clone()),
          };
          find_end(map, next_pos)
        })
        .sum();
      map.insert(pos, Path::Resolved(result));
      result
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 11;
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_11_b() {
    const EXAMPLE_ANSWER: Option<u64> = Some(2);
    const ANSWER: Option<u64> = Some(331468292364745);
    match utils::run_method::<u64>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_11_b(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
