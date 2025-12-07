use std::str::Chars;

extern crate test;

#[derive(Debug)]
enum Op {
  Multiply,
  Add
}

pub fn main(contents: String) -> u64 {
  do_maths(contents)
}

fn do_maths(contents: String) -> u64 {
  let mut lines: Vec<Chars> = contents
    .lines()
    .map(|line| line.chars())
    .collect();
  let mut op_line = lines.remove(lines.len()-1);

  let mut ans: u64 = 0;
  let mut current_num: u64 = 0;
  let mut current_op: Op = Op::Add;
  while let Some(op) = op_line.next() {
    if op == '*' {
      current_num = 1;
      current_op = Op::Multiply;
    } else if op == '+' {
      current_num = 0;
      current_op = Op::Add;
    }
    let new_num = lines
      .iter_mut()
      .filter_map(|it| it.next().unwrap().to_digit(10))
      .fold(0u64, |acc, c| acc * 10 + c as u64);
    if new_num == 0 {
      ans += current_num;
    } else {
      match current_op {
        Op::Add => current_num += new_num,
        Op::Multiply => current_num *= new_num
      }
    }
  }
  ans + current_num
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 6;
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_06_b() {
    const EXAMPLE_ANSWER: Option<u64> = Some(3263827);
    const ANSWER: Option<u64> = Some(9194682052782);
    match utils::run_method::<u64>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_06_b(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
