use std::collections::HashMap;

extern crate test;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Path {
  Unresolved(Vec<String>),
  Resolved(u64)
}

pub fn main(contents: String) -> u64 {
  maze_it_out(contents)
}

fn maze_it_out(contents: String) -> u64 {
  let mut maze = contents
    .lines()
    .map(|line| {
      let (start, end) = line.split_once(": ").unwrap();
      (start.to_string(), Path::Unresolved(end.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>()))
    })
    .collect::<HashMap<String, Path>>();
  maze.insert("out".to_string(), Path::Resolved(1));
  find_end(&mut maze, "you".to_string())
}

fn find_end(map: &mut HashMap<String, Path>, pos: String) -> u64 {
  match map[&pos].clone() {
    Path::Resolved(existing) => existing,
    Path::Unresolved(possibilities) => {
      let result = possibilities
        .iter()
        .map(|p| find_end(map, p.clone()))
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
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_11_a() {
    const EXAMPLE_ANSWER: Option<u64> = Some(5);
    const ANSWER: Option<u64> = Some(428);
    match utils::run_method::<u64>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_11_a(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
