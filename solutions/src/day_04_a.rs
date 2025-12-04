extern crate test;

pub fn main(contents: String) -> u32 {
  get_toilet_papers(contents)
}

fn get_toilet_papers(contents: String) -> u32 {
  let map = contents
    .lines()
    .map(|line| line.chars().collect::<Vec<char>>())
    .collect::<Vec<Vec<char>>>();
  let mut rolls: u32 = 0;
  for i in 0..map.len() {
    for j in 0..map[0].len() {
      if map[i][j] == '@' {
        let mut surrounding: u8 = 0;
        if i > 0 {
          if j > 0 {
            if map[i-1][j-1] == '@' {
              surrounding += 1;
            }
          }
          if map[i-1][j] == '@' {
            surrounding += 1;
          }
          if j < map[i].len() - 1 {
            if map[i-1][j+1] == '@' {
              surrounding += 1;
            }
          }
        }
        if j > 0 {
          if map[i][j-1] == '@' {
            surrounding += 1;
          }
        }
        if j < map[i].len() - 1 {
          if map[i][j+1] == '@' {
            surrounding += 1;
          }
        }
        if i < map.len() - 1 {
          if j > 0 {
            if map[i+1][j-1] == '@' {
              surrounding += 1;
            }
          }
          if map[i+1][j] == '@' {
            surrounding += 1;
          }
          if j < map[i].len() - 1 {
            if map[i+1][j+1] == '@' {
              surrounding += 1;
            }
          }
        }
        if surrounding < 4 {
          rolls += 1;
        }
      }
    }
  }
  rolls
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 4;
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_04_a() {
    const EXAMPLE_ANSWER: Option<u32> = Some(13);
    const ANSWER: Option<u32> = None;
    match utils::run_method::<u32>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_04_a(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
