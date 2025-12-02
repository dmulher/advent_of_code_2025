extern crate test;

pub fn main(contents: String) -> u16 {
  get_password(contents)
}

#[derive(Debug)]
enum Command{
    Left(u16),
    Right(u16)
}

#[derive(Debug)]
struct SafeState {
  clicks: u16,
  position: u16
}

fn get_password(contents: String) -> u16 {
  contents
    .lines()
    .map(str_to_command)
    .fold(SafeState {clicks: 0u16, position: 50u16}, move_dial)
    .clicks
}

fn str_to_command(line: &str) -> Command {
  if let Some(left) = line.strip_prefix("L") {
    Command::Left(left.parse::<u16>().unwrap())
  } else if let Some(right) = line.strip_prefix("R") {
    Command::Right(right.parse::<u16>().unwrap())
  } else {
    Command::Left(0)
  }
}

fn move_dial(state: SafeState, command: Command) -> SafeState {
  match command {
    Command::Left(d) => {
      if d > state.position {
        let remainder = d - state.position;
        SafeState {
          clicks: state.clicks + if (remainder % 100) == 0 {1} else {0},
          position: if (remainder % 100) == 0 {0} else {100 - (remainder % 100)}
        }
      } else if d < state.position {
        SafeState { clicks: state.clicks, position: state.position - d }
      } else {
        SafeState { clicks: state.clicks + 1, position: 0 }
      }
    },
    Command::Right(d) => {
      let new_pos = d + state.position;
      SafeState { clicks: state.clicks + if new_pos % 100 == 0 {1} else {0}, position: new_pos % 100 }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 1;
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_01_a() {
    const EXAMPLE_ANSWER: Option<u16> = Some(3);
    const ANSWER: Option<u16> = Some(1154);
    match utils::run_method::<u16>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_01_a(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
