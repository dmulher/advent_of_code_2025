use std::collections::{HashMap, VecDeque};

extern crate test;

pub fn main(contents: String) -> u16 {
  largest_rectangle(contents)
}

fn largest_rectangle(contents: String) -> u16 {
  contents
    .lines()
    .map(|line| {
      let (expected_lines, rest) = line
        .strip_prefix('[')
        .unwrap()
        .split_once("] (")
        .unwrap();
      let expected_lights: u16 = expected_lines.chars().map(|c| c == '#').rev().fold(0, |acc, c| acc * 2 + if c {1} else {0});
      if expected_lights == 0 {
        return 0;
      }
      let (buttons_str, _) = rest.split_once(") {").unwrap();
      let buttons = buttons_str.split(") (").map(|button| button.split(",").map(|b| 2u16.pow(b.parse::<u32>().unwrap())).sum()).collect::<Vec<u16>>();

      // state = current lights, buttons pressed, 
      let mut seen_nodes: HashMap::<u16, u16> = HashMap::new();
      seen_nodes.insert(0, 0);
      let mut node_array: VecDeque<(u16, u16)> = VecDeque::new();
      node_array.push_back((0, 0));
      while let Some((curr_state, presses)) = node_array.pop_front() {
        for button in buttons.iter() {
          let new_state = curr_state ^ button;
          if new_state == expected_lights {
            return (presses + 1) as u16;
          } else if seen_nodes.contains_key(&new_state) {
            continue;
          } else {
            seen_nodes.insert(new_state, presses + 1);
            node_array.push_back((new_state, presses + 1));
          }
        }
      }
      0
    })
  .sum()
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 10;
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_10_a() {
    const EXAMPLE_ANSWER: Option<u16> = Some(7);
    const ANSWER: Option<u16> = Some(444);
    match utils::run_method::<u16>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_10_a(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
