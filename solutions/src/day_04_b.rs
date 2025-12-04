extern crate test;

pub fn main(contents: String) -> u32 {
  get_toilet_papers(contents)
}

#[derive(Debug, PartialEq, Eq)]
enum Spot {
  None,
  Roll(u8)
}

fn get_toilet_papers(contents: String) -> u32 {
  let mut map = contents
    .lines()
    .map(|line| line.chars().map(|c| if c == '@' {Spot::Roll(0)} else {Spot::None}).collect::<Vec<Spot>>())
    .collect::<Vec<Vec<Spot>>>();
  let mut rolls: u32 = 0;
  for i in 0..map.len() {
    for j in 0..map[0].len() {
      if map[i][j] == Spot::None {
        continue;
      }
      let mut surrounding: u8 = 0;
      if i > 0 {
        if j > 0 {
          if map[i-1][j-1] != Spot::None {
            surrounding += 1;
          }
        }
        if map[i-1][j] != Spot::None {
          surrounding += 1;
        }
        if j < map[i].len() - 1 {
          if map[i-1][j+1] != Spot::None {
            surrounding += 1;
          }
        }
      }
      if j > 0 {
        if map[i][j-1] != Spot::None {
          surrounding += 1;
        }
      }
      if j < map[i].len() - 1 {
        if map[i][j+1] != Spot::None {
          surrounding += 1;
        }
      }
      if i < map.len() - 1 {
        if j > 0 {
          if map[i+1][j-1] != Spot::None {
            surrounding += 1;
          }
        }
        if map[i+1][j] != Spot::None {
          surrounding += 1;
        }
        if j < map[i].len() - 1 {
          if map[i+1][j+1] != Spot::None {
            surrounding += 1;
          }
        }
      }
      if surrounding < 4 {
        rolls += change_adjacency(&mut map, i, j, i, j);
      } else {
        map[i][j] = Spot::Roll(surrounding);
      }
    }
  }
  for line in map.iter() {
    println!("{:?}", line);
  }
  rolls
}

fn change_adjacency(map: &mut Vec<Vec<Spot>>, i: usize, j: usize, oi: usize, oj: usize) -> u32 {
  map[i][j] = Spot::None;
  let mut changes: u32 = 1;
  if j > 0 {
    changes += check_for_cascade(map, i, j-1, oi, oj);
  }
  if i > 0 {
    if j > 0 {
      changes += check_for_cascade(map, i-1, j-1, oi, oj);
    }
    changes += check_for_cascade(map, i-1, j, oi, oj);
    if j < map[i].len() - 1 {
      changes += check_for_cascade(map, i-1, j+1, oi, oj);
    }
  }
  if (i == oi && oj > 1 && j < oj - 2) || (i < oi && j < map[i].len() - 1) {
    changes += check_for_cascade(map, i, j+1, oi, oj);
  }
  if i < oi {
    if j > 0 && (i < oi - 1 || j - 1 < oj) {
      changes += check_for_cascade(map, i+1, j-1, oi, oj);
    }
    if i < oi - 1 {
      changes += check_for_cascade(map, i+1, j, oi, oj);
    }
    if (i < oi - 1 && j < map[i].len() - 1) || j < oj {
      changes += check_for_cascade(map, i+1, j+1, oi, oj);
    }
  }
  changes
}

fn check_for_cascade(map: &mut Vec<Vec<Spot>>, i: usize, j: usize, oi: usize, oj: usize) -> u32 {
  let mut changes = 0u32;
  if let Spot::Roll(x) = map[i][j] {
    if x == 4 {
      changes += change_adjacency(map, i, j, oi, oj);
    } else {
      map[i][j] = Spot::Roll(x - 1);
    }
  }
  changes
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 4;
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_04_b() {
    const EXAMPLE_ANSWER: Option<u32> = Some(43);
    const ANSWER: Option<u32> = None;
    match utils::run_method::<u32>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  // #[bench]
  // fn bench_day_04_b(b: &mut Bencher) {
  //   let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
  //   b.iter(|| main(input.clone()));
  // }
}
