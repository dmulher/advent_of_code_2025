use std::collections::HashMap;

extern crate test;

pub fn main(contents: String) -> u64 {
  largest_rectangle(contents)
}

fn largest_rectangle(contents: String) -> u64 {
  let mut map: HashMap<u32, (u32, u32)> = HashMap::new();
  contents
    .lines()
    .map(|l| {
      let (a, b) = l.split_once(',').unwrap();
      (b.parse::<u32>().unwrap(), a.parse::<u32>().unwrap())  // y, x
    })
    .for_each(|(x, y)| {
      match map.get(&x) {
        None => {map.insert(x, (y, y));},
        Some((low, high)) => if y < *low {map.insert(x, (y, *high));} else if y > *high {map.insert(x, (*low, y));},
      };
    });
  let keys = map.keys().collect::<Vec<&u32>>();
  let mut highest = 0;
  for i in 0..keys.len() - 1 {
    for j in i..keys.len() {
      let i_y = keys[i];
      let j_y = keys[j];
      let height = if i_y > j_y {i_y - j_y} else {j_y - i_y};
      let (i_left, i_right) = map[i_y];
      let (j_left, j_right) = map[j_y];
      let greatest_width = (if i_right > j_left {i_right - j_left} else {0}).max(if i_left < j_right {j_right - i_left} else {0});
      highest = highest.max((height + 1) as u64 * (greatest_width + 1) as u64)
    }
  }
  highest
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 9;
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_09_a() {
    const EXAMPLE_ANSWER: Option<u64> = Some(50);
    const ANSWER: Option<u64> = Some(4774877510);
    match utils::run_method::<u64>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_09_a(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
