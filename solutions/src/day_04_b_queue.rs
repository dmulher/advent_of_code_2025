use std::collections::VecDeque;

extern crate test;

pub fn main(contents: String) -> u16 {
  get_toilet_papers(contents)
}

fn get_toilet_papers(contents: String) -> u16 {
  let mut map = create_map(contents);
  let mut queue = set_initial_adjacency(&mut map);
  let ans = remove_rolls(&mut queue, &mut map);
  ans
}

fn create_map(contents: String) -> Vec<Vec<Option<u8>>> {
  contents
    .lines()
    .map(|line| line.chars().map(|c| if c == '@' {Some(0)} else {None}).collect::<Vec<Option<u8>>>())
    .collect::<Vec<Vec<Option<u8>>>>()
}

fn set_initial_adjacency(map: &mut Vec<Vec<Option<u8>>>) -> VecDeque<(usize, usize)> {
  let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
  for i in 0..map.len() {
    for j in 0..map[0].len() {
      if map[i][j].is_none() {
        continue;
      }
      let mut surrounding: u8 = 0;
      if i > 0 {
        if j > 0 {
          if map[i-1][j-1].is_some() {
            surrounding += 1;
          }
        }
        if map[i-1][j].is_some() {
          surrounding += 1;
        }
        if j < map[i].len() - 1 {
          if map[i-1][j+1].is_some() {
            surrounding += 1;
          }
        }
      }
      if j > 0 {
        if map[i][j-1].is_some() {
          surrounding += 1;
        }
      }
      if j < map[i].len() - 1 {
        if map[i][j+1].is_some() {
          surrounding += 1;
        }
      }
      if i < map.len() - 1 {
        if j > 0 {
          if map[i+1][j-1].is_some() {
            surrounding += 1;
          }
        }
        if map[i+1][j].is_some() {
          surrounding += 1;
        }
        if j < map[i].len() - 1 {
          if map[i+1][j+1].is_some() {
            surrounding += 1;
          }
        }
      }
      map[i][j] = Some(surrounding);
      if surrounding < 4 {
        queue.push_back((i, j));
      }
    }
  }
  queue
}

fn remove_rolls(queue: &mut VecDeque<(usize, usize)>, map: &mut Vec<Vec<Option<u8>>>) -> u16 {
  let mut rolls_removed = 0u16;
  while let Some((i, j)) = queue.pop_front() {
    if let Some(x) = map[i][j] {
      if x < 5 {
        map[i][j] = None;
        rolls_removed += 1;
        add_checks_to_queue(queue, map, i, j);
      } else {
        map[i][j] = Some(x - 1);
      }
    }
  }
  rolls_removed
}

fn add_checks_to_queue(queue: &mut VecDeque<(usize, usize)>, map: &mut Vec<Vec<Option<u8>>>, i: usize, j: usize) -> () {
  if i > 0 {
    if j > 0 && map[i-1][j-1].is_some() {
      queue.push_back((i-1, j-1));
    }
    if map[i-1][j].is_some() {
      queue.push_back((i-1, j));
    }
    if j < map[i].len() - 1 && map[i-1][j+1].is_some() {
      queue.push_back((i-1, j+1));
    }
  }
  if j > 0 && map[i][j-1].is_some() {
    queue.push_back((i, j-1));
  }
  if j < map[i].len() - 1 && map[i][j+1].is_some() {
    queue.push_back((i, j+1));
  }
  if i < map.len() - 1 {
    if j > 0 && map[i+1][j-1].is_some() {
      queue.push_back((i+1, j-1));
    }
    if map[i+1][j].is_some() {
      queue.push_back((i+1, j));
    }
    if j < map[i].len() - 1 && map[i+1][j+1].is_some() {
      queue.push_back((i+1, j+1));
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 4;
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_04_b_queue() {
    const EXAMPLE_ANSWER: Option<u16> = Some(43);
    const ANSWER: Option<u16> = Some(9083);
    match utils::run_method::<u16>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_04_b_queue(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
