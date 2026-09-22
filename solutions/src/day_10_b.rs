extern crate test;
use utils::maths::Frac;

pub fn main(contents: String) -> u64 {
  press_buttons(contents)
}

fn press_buttons(contents: String) -> u64 {
  contents
    .lines()
    .enumerate()
    .map(|(i, line)| {
      println!("--------------------STARTING EQUATION {}--------------------", i);
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

      let joltage = joltage_str.strip_suffix('}').unwrap().split(',').map(|c| c.parse::<i16>().unwrap()).collect::<Vec<i16>>();
      let joltage_len = joltage.len();

      let equations: Vec<(Vec<Frac>, Frac)> = (0..joltage_len)
        .map(|i| {
          let eq = buttons
            .iter()
            .map(|b| if b.contains(&(i as u16)) {Frac::new(1, 1)} else {Frac::new(0, 1)})
            .collect();
          (eq, Frac::new(joltage[i], 1))
        })
        .collect();
      println!("equations:");
      for eq in &equations {
        println!("{:?}", eq);
      }

      // let options = get_gaussian_options(equations, 0, buttons.len());
      // println!("ALL OPTIONS PRESENT AND ACCOUNTED FOR:");
      let ans = gauss_eliminate(equations, 0, buttons.len()).unwrap();
      println!("{}", ans);
      ans
    })
  .sum()
}

fn gauss_eliminate(mut equations: Vec<(Vec<Frac>, Frac)>, col: usize, button_count: usize) -> Option<u64> {
  if col == button_count - 1 || col == equations.len() - 1 {
    let (mut final_eq, mut final_ans) = equations[col].clone();
    let div = final_eq[col];
    if !div.is_zero() {
      for i in col..final_eq.len() {
        final_eq[i] = final_eq[i] / div;
      }
      final_ans = final_ans / div;
    }
    equations[col] = (final_eq.clone(), final_ans);
    (final_eq.len()..equations.len())
      .for_each(|i| {
        let (mut curr_eq, curr_ans) = equations[i].clone();
        let diff = curr_eq[col];
        curr_eq[col] = curr_eq[col] - diff * final_eq[col];
        equations[i] = (curr_eq, curr_ans - diff * final_ans);
      });

    let mut all_possible_solutions = vec![vec![None; final_eq.len()]];
    let mut attempts = 0;
    let mut current_min = None;
    while !all_possible_solutions.is_empty() && attempts < equations.len() {
      current_min = all_possible_solutions
        .iter()
        .map(|a| optional_sum(&a))
        .fold(None, |acc, small| {
          match (acc, small) {
            (None, None) => None,                                                                                                                                                                                                     
            (Some(_), None) => acc,
            (None, Some(_)) => small,
            (Some(a), Some(b)) => Some(a.min(b))
          }
        });

      all_possible_solutions = all_possible_solutions
        .iter()
        .filter(|b| b.iter().any(|b| b.is_none()))
        .flat_map(|solution| {
          get_solutions(&equations, &solution, equations.len()-1)
        })
        .collect();
      attempts += 1;
    }
    let ans = solve(all_possible_solutions);

    match (ans, current_min) {
      (None, None) => None,
      (Some(_), None) => ans,
      (None, Some(_)) => current_min,
      (Some(a), Some(b)) => Some(a.min(b))
    }
  } else if (col..equations.len()).all(|i| equations[i].0[col].is_zero()) {
    gauss_eliminate(equations, col + 1, button_count)
  } else {
    (col..equations.len())
      // Optimise: Remove all options that are 0 at this col
      .filter(|rowi| !equations[*rowi].0[col].is_zero())
      .map(|rowi| {
        let mut new_equations = equations.clone();
        // Bring variable at this col to 1
        let (mut munged_chosen_eq, mut munged_chosen_ans) = new_equations[rowi].clone();
        let div = munged_chosen_eq[col];
        for i in col..munged_chosen_eq.len() {
          munged_chosen_eq[i] = munged_chosen_eq[i] / div;
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
              curr_eq[cj] = curr_eq[cj] - diff * munged_chosen_eq[cj];
            }
            new_equations[rowj] = (curr_eq, curr_ans - diff * munged_chosen_ans);
          });
        gauss_eliminate(new_equations, col + 1, button_count)
      })
      .filter_map(|thing| thing)
      .next()
      // .fold(None, |acc, solution| {
      //   match (acc, solution) {
      //     (None, a) => a,
      //     (a, None) => a,
      //     (Some(a), Some(b)) => Some(a.min(b))
      //   }
      // })
  }
}

fn optional_sum(op: &Vec<Option<u64>>) -> Option<u64> {
  op
    .iter()
    .fold(Some(0), |acc, optional| match (acc, optional) {
      (None, _) => None,
      (_, None) => None,
      (Some(a), Some(b)) => Some(a + *b)
    })
}

fn solve(solutions: Vec<Vec<Option<u64>>>) -> Option<u64> {
  solutions
    .into_iter()
    .map(|solution| solution.into_iter().map(|a| a.unwrap_or(0)).sum())
    .min()
}

fn get_solutions(equations: &Vec<(Vec<Frac>, Frac)>, solved_variables: &Vec<Option<u64>>, to_solve: usize) -> Vec<Vec<Option<u64>>> {
  let (eq, ans) = &equations[to_solve];
  let current_ans = *ans - (0..solved_variables.len()).map(|i| eq[i] * solved_variables[i].unwrap_or(0) as i16).sum::<Frac>();
  let vars_to_solve_iter = (0..eq.len())
    .filter(|i| solved_variables[*i].is_none() && !eq[*i].is_zero());
  let coeffs = vars_to_solve_iter.clone().map(|i| eq[i]).collect();

  let vars_to_solve_iter = vars_to_solve_iter.enumerate();
  let all_options = get_all_ranges(coeffs, current_ans);
  if all_options.is_empty() {
    if to_solve == 0 {
      vec![solved_variables.clone()]
    } else {
      get_solutions(equations, solved_variables, to_solve - 1)
    }
  } else {
    all_options
      .into_iter()
      .flat_map(|option| {
        let mut new_solved = solved_variables.clone();
        for (i, j) in vars_to_solve_iter.clone() {
          new_solved[j] = Some(option[i] as u64);
        }
        if to_solve == 0 {
          vec![new_solved]
        } else {
          get_solutions(equations, &new_solved, to_solve - 1)
        }
      })
      .collect::<Vec<Vec<Option<u64>>>>()
  }
}

fn get_all_ranges(coeffs: Vec<Frac>, target_num: Frac) -> Vec<Vec<u32>> {
  if coeffs.is_empty() {
    vec![]
  } else if coeffs.len() == 1 {
    let max_i = target_num / coeffs[0];
    if !max_i.is_whole() || max_i.is_negative() {
      vec![]
    } else {
      vec![vec![max_i.floor() as u32]]
    }
  } else {
    let last_coeff = coeffs[coeffs.len()-1];
    let max_i = (target_num / last_coeff).floor();
    if max_i < 0 {
      vec![]
    } else {
      (0..=max_i)
        .flat_map(|i| {
          let new_target = target_num - last_coeff * i;
          let new_coeffs: Vec<Frac> = coeffs.clone().into_iter().take(coeffs.len()-1).collect();
          get_all_ranges(new_coeffs, new_target)
            .into_iter()
            .map(|mut new_range| {
              new_range.push(i as u32);
              new_range
            })
            .collect::<Vec<Vec<u32>>>()
        })
        .collect()
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  // use test::Bencher;
  // use utils::read_file_to_string;

  const DAY: u8 = 10;
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_10_b() {
    const EXAMPLE_ANSWER: Option<u64> = Some(33);
    const ANSWER: Option<u64> = None;  // 16463 was too low
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
