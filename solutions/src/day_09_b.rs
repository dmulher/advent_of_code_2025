extern crate test;

enum Line {
  Horizontal(u32, u32, u32),
  Vertical(u32, u32, u32)
}

pub fn main(contents: String) -> u64 {
  largest_rectangle(contents)
}

fn largest_rectangle(contents: String) -> u64 {
  let coords = contents
    .lines()
    .map(|l| {
      let (a, b) = l.split_once(',').unwrap();
      (b.parse::<u32>().unwrap(), a.parse::<u32>().unwrap())  // y, x
    })
    .collect::<Vec<(u32, u32)>>();
  let mut lines: Vec<Line> = Vec::new();
  let mut last_coord = coords[coords.len()-1];
  coords
    .iter()
    .for_each(|coord| {
      if coord.0 == last_coord.0 {
        lines.push(Line::Horizontal(coord.0, last_coord.1.min(coord.1), last_coord.1.max(coord.1)));
      } else {
        lines.push(Line::Vertical(coord.1, last_coord.0.min(coord.0), last_coord.0.max(coord.0)));
      }
      last_coord = *coord
    });
  let mut largest = 0;
  for i in 0..coords.len() - 1 {
    for j in i..coords.len() {
      let top = coords[i].0.min(coords[j].0);
      let bottom = coords[i].0.max(coords[j].0);
      let left = coords[i].1.min(coords[j].1);
      let right = coords[i].1.max(coords[j].1);
      let area = (bottom - top + 1) as u64 * (right - left + 1) as u64;
      if area <= largest {
        continue;
      }
      if lines.iter().all(|line| {
        match line {
          Line::Horizontal(y, x1, x2) => {
            if y > &top && y < &bottom {
              if x1 < &right && x2 > &left {
                return false;
              }
            }
          },
          Line::Vertical(x, y1, y2) => {
            if x > &left && x < &right {
              if y1 < &bottom && y2 > &top {
                return false;
              }
            }
          }
        }
        true
      }) {
        largest = area;
      }
    }
  }
  largest
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 9;
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_09_b() {
    const EXAMPLE_ANSWER: Option<u64> = Some(24);
    const ANSWER: Option<u64> = Some(1560475800);
    match utils::run_method::<u64>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_09_b(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
