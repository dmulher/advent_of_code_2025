use std::{str::SplitWhitespace};

extern crate test;

pub fn main(contents: String) -> u64 {
  do_maths(contents)
}

fn do_maths(contents: String) -> u64 {
  let mut lines: Vec<SplitWhitespace> = contents
    .lines()
    .map(|line| line.split_whitespace())
    .collect();

  let mut ans: u64 = 0;
  let mut op_line = lines.remove(lines.len()-1);
  while let Some(op) = op_line.next() {
    ans += match op {
      "+" => {
        lines.iter_mut().fold(0u64, |acc, it| acc + it.next().unwrap().parse::<u64>().unwrap())
      },
      "*" => {
        lines.iter_mut().fold(1u64, |acc, it| acc * it.next().unwrap().parse::<u64>().unwrap())
      },
      _ => {
        0
      }
    };
  }
  ans
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 6;
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_06_a() {
    const EXAMPLE_ANSWER: Option<u64> = Some(4277556);
    const ANSWER: Option<u64> = Some(6295830249262);
    match utils::run_method::<u64>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_06_a(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
