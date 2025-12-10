extern crate test;

pub fn main(contents: String) -> u64 {
  largest_rectangle(contents)
}

fn largest_rectangle(contents: String) -> u64 {
  contents
    .lines()
    .map(|line| {
      let (_, rest) = line
        .split_once("] (")
        .unwrap();
      let (buttons_str, joltage_str) = rest.split_once(") {").unwrap();
      let buttons: Vec<Vec<u16>> = buttons_str
        .split(") (")
        .map(|button| button
          .split(",")
          .map(|b| b.parse::<u16>().unwrap())
          .collect::<Vec<u16>>())
        .collect();

      let joltage = joltage_str.strip_suffix('}').unwrap().split(',').map(|c| c.parse::<u16>().unwrap()).collect::<Vec<u16>>();
      let joltage_len = joltage.len();

      let equations: Vec<(Vec<f64>, f64)> = (0..joltage_len)
        .map(|i| {
          let eq = buttons
            .iter()
            .map(|b| if b.contains(&(i as u16)) {1f64} else {0f64})
            .collect();
          (eq, joltage[i] as f64)
        })
        .collect();
      println!("equations:");
      for eq in &equations {
        println!("{:?}", eq);
      }

      gauss_eliminate(equations, 0, buttons.len()).unwrap()
    })
  .sum()
}

fn gauss_eliminate(mut equations: Vec<(Vec<f64>, f64)>, col: usize, button_count: usize) -> Option<u64> {
  println!("equations before doing col {}:", col);
  for eq in &equations {
    println!("{:?}", eq);
  }
  if col == button_count - 1 || col == equations.len() - 1 {
    println!("Breaking free");
    let div = equations[col].0[col];
    if div != 0.0 {
      for i in col..equations[col].0.len() {
        equations[col].0[i] /= div;
      }
      equations[col].1 /= div;
    }

    solve(&equations)
  } else {
    (col..equations.len())
      // Optimise: Remove all options that are 0 at this col
      .filter(|rowi| equations[*rowi].0[col] != 0.0)
      .map(|rowi| {
        let mut new_equations = equations.clone();
        // Bring variable at this col to 1
        let (mut munged_chosen_eq, mut munged_chosen_ans) = new_equations[rowi].clone();
        let div = munged_chosen_eq[col];
        for i in col..munged_chosen_eq.len() {
          munged_chosen_eq[i] /= div;
        }
        munged_chosen_ans = munged_chosen_ans / div;
        new_equations[rowi] = (munged_chosen_eq.clone(), munged_chosen_ans);
        // Shift equations
        if rowi > col {
          new_equations.swap(col, rowi);
        }
        // Eliminate
        (col+1..new_equations.len())
          .for_each(|rowj| {
            let (mut curr_eq, curr_ans) = new_equations[rowj].clone();
            let diff = curr_eq[col];
            for cj in col..curr_eq.len() {
              curr_eq[cj] -= diff * munged_chosen_eq[cj];
            }
            new_equations[rowj] = (curr_eq, curr_ans - diff * munged_chosen_ans);
          });
        gauss_eliminate(new_equations, col + 1, button_count)
      })
      .fold(None, |acc, solution| {
        match (acc, solution) {
          (None, a) => a,
          (a, None) => a,
          (Some(a), Some(b)) => Some(a.min(b))
        }
      })
  }
}

fn solve(equations: &Vec<(Vec<f64>, f64)>) -> Option<u64> {
  let mut vars = vec![0f64; equations.len()];
  for i in (0..equations.len()).rev() {
    let (eq, ans) = &equations[i];
    let new_var = ans - (i+1..equations.len()).rev().map(|j| eq[j] * vars[j]).sum::<f64>();
    if new_var.round() != new_var {
      return None;
    }
    vars[i] = new_var.round()
  }
  Some(vars.iter().map(|v| *v as u64).sum())
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 10;
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_10_b() {
    const EXAMPLE_ANSWER: Option<u64> = Some(33);
    const ANSWER: Option<u64> = None;
    match utils::run_method::<u64>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  // #[bench]
  // fn bench_day_10_b(b: &mut Bencher) {
  //   let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
  //   b.iter(|| main(input.clone()));
  // }
}
