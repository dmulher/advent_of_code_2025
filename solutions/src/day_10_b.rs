extern crate test;
use std::collections::BinaryHeap;

use utils::maths::Frac;

struct Equation {
  coeffs: Vec<Frac>,
  ans: Frac
}

impl Equation {
  pub fn reduce(&mut self, col: usize) -> () {
    if !self.coeffs[col].is_one() {
      let div = self.coeffs[col];
      for i in 0..self.coeffs.len() {
        self.coeffs[i] = self.coeffs[i] / div;
      }
      self.ans = self.ans / div;
    }
  }

  pub fn subtract_by(&mut self, other: Self, col: usize) -> () {
    let mult = self.coeffs[col];
    if !mult.is_zero() {
      for i in col..self.coeffs.len() {
        self.coeffs[i] = self.coeffs[i] - other.coeffs[i] * mult;
      }
      self.ans = self.ans - other.ans * mult
    }
  }

  pub fn is_all_zero(&self) -> bool {
    self.ans.is_zero() && self.coeffs.iter().all(|c| c.is_zero())
  }
}


pub fn main(contents: String) -> u64 {
  press_buttons(contents)
}

fn press_buttons(contents: String) -> u64 {
  contents
    .lines()
    .map(|line| {
      let (buttons, joltage) = parse_line(line);

      let width = buttons.len();
      let height = joltage.len();

      let equations: Vec<Equation> = (0..height)
        .map(|i| {
          let coeffs = buttons
            .iter()
            .map(|b| if b & (1 << i) > 0 {Frac::new(1, 1)} else {Frac::new(0, 1)})
            .collect();
          Equation {coeffs, ans: joltage[i]}
        })
        .collect();

      let gaussian_equations = gauss_eliminate(equations, height, width);
      let height = gaussian_equations.len();

      // For the extra columns, figure out the last ones 
      // let ans = old_gauss_eliminate(equations, height, width).unwrap();
      let solutions = get_solutions(&gaussian_equations, &vec![], height-1);
      solve(solutions).unwrap()
    })
  .sum()
}

fn gauss_eliminate(mut equations: Vec<Equation>, height: usize, width: usize) -> Vec<Equation> {
  for curr_col in 0..height.min(width) {
    let pivot_row = equations.iter().enumerate().filter(|eq| !eq.1.coeffs[curr_col].is_zero()).next();
    if let Some((pivot_row_index, _)) = pivot_row {
      if curr_col != pivot_row_index {
        equations.swap(curr_col, pivot_row_index);
      }

      equations[curr_col].reduce(curr_col);
    }
  }

  equations.into_iter().filter(|l| !l.is_all_zero()).collect()
}

fn solve(mut equations: Vec<Equation>, width: usize, height: usize) -> u32 {
  // let attempt_gremlin: BinaryHeap<Equation> = BinaryHeap::new();
  let a = ((width..height).map(|c| {
            let buttons = Vec::<u32>::new();
            joltages.iter().enumerate().filter_map(move |(n, &joltage)|
                if buttons[c] & (1 << n) != 0 { Some(joltage.to_integer() as u32) } else { None }
            ).min().unwrap()
        })).collect();
}

// fn old_gauss_eliminate(mut equations: Vec<(Vec<Frac>, Frac)>, col: usize, button_count: usize) -> Option<u64> {
//   if col == button_count - 1 || col == equations.len() - 1 {
//     let (mut final_eq, mut final_ans) = equations[col].clone();
//     let div = final_eq[col];
//     if !div.is_zero() {
//       for i in (col..final_eq.len()).rev() {
//         final_eq[i] = final_eq[i] / div;
//       }
//       final_ans = final_ans / div;
//     }
//     equations[col] = (final_eq.clone(), final_ans);
//     (final_eq.len()..equations.len())
//       .for_each(|i| {
//         let (mut curr_eq, curr_ans) = equations[i].clone();
//         let diff = curr_eq[col];
//         curr_eq[col] = curr_eq[col] - diff * final_eq[col];
//         equations[i] = (curr_eq, curr_ans - diff * final_ans);
//       });

//     let mut all_possible_solutions = vec![vec![None; final_eq.len()]];
//     let mut attempts = 0;
//     let mut current_min = None;
//     while !all_possible_solutions.is_empty() && attempts < equations.len() {
//       current_min = all_possible_solutions
//         .iter()
//         .map(|a| optional_sum(&a))
//         .fold(None, |acc, small| {
//           match (acc, small) {
//             (None, None) => None,                                                                                                                                                                                                     
//             (Some(_), None) => acc,
//             (None, Some(_)) => small,
//             (Some(a), Some(b)) => Some(a.min(b))
//           }
//         });

//       all_possible_solutions = all_possible_solutions
//         .iter()
//         .filter(|b| b.iter().any(|b| b.is_none()))
//         .flat_map(|solution| {
//           get_solutions(&equations, &solution, equations.len()-1)
//         })
//         .collect();
//       attempts += 1;
//     }
//     let ans = solve(all_possible_solutions);

//     match (ans, current_min) {
//       (None, None) => None,
//       (Some(_), None) => ans,
//       (None, Some(_)) => current_min,
//       (Some(a), Some(b)) => Some(a.min(b))
//     }
//   } else if (col..equations.len()).all(|i| equations[i].0[col].is_zero()) {
//     old_gauss_eliminate(equations, col + 1, button_count)
//   } else {
//     (col..equations.len())
//       // Optimise: Remove all options that are 0 at this col
//       .filter(|rowi| !equations[*rowi].0[col].is_zero())
//       .map(|rowi| {
//         let mut new_equations = equations.clone();
//         // Bring variable at this col to 1
//         let (mut munged_chosen_eq, mut munged_chosen_ans) = new_equations[rowi].clone();
//         let div = munged_chosen_eq[col];
//         for i in col..munged_chosen_eq.len() {
//           munged_chosen_eq[i] = munged_chosen_eq[i] / div;
//         }
//         munged_chosen_ans = munged_chosen_ans / div;
//         new_equations[rowi] = (munged_chosen_eq.clone(), munged_chosen_ans);
//         // Shift equations
//         if rowi > col {
//           new_equations.swap(col, rowi);
//         }
//         // Eliminate
//         (col+1..new_equations.len())
//           .for_each(|rowj| {
//             let (mut curr_eq, curr_ans) = new_equations[rowj].clone();
//             let diff = curr_eq[col];
//             for cj in col..curr_eq.len() {
//               curr_eq[cj] = curr_eq[cj] - diff * munged_chosen_eq[cj];
//             }
//             new_equations[rowj] = (curr_eq, curr_ans - diff * munged_chosen_ans);
//           });
//         old_gauss_eliminate(new_equations, col + 1, button_count)
//       })
//       .filter_map(|thing| thing)
//       .next()
//       // .fold(None, |acc, solution| {
//       //   match (acc, solution) {
//       //     (None, a) => a,
//       //     (a, None) => a,
//       //     (Some(a), Some(b)) => Some(a.min(b))
//       //   }
//       // })
//   }
// }

// fn optional_sum(op: &Vec<Option<u64>>) -> Option<u64> {
//   op
//     .iter()
//     .fold(Some(0), |acc, optional| match (acc, optional) {
//       (None, _) => None,
//       (_, None) => None,
//       (Some(a), Some(b)) => Some(a + *b)
//     })
// }

// fn solve(solutions: Vec<Vec<Option<u64>>>) -> Option<u64> {
//   solutions
//     .into_iter()
//     .map(|solution| solution.into_iter().map(|a| a.unwrap_or(0)).sum())
//     .min()
// }

// fn get_solutions(equations: &Vec<Equation>, solved_variables: &Vec<Option<u64>>, to_solve: usize) -> Vec<Vec<Option<u64>>> {
//   let equation = &equations[to_solve];
//   let eq = &equation.coeffs;
//   let ans = equation.ans;
//   let current_ans = ans - (0..solved_variables.len()).map(|i| eq[i] * solved_variables[i].unwrap_or(0) as i16).sum::<Frac>();
//   let vars_to_solve_iter = (0..eq.len())
//     .filter(|i| solved_variables[*i].is_none() && !eq[*i].is_zero());
//   let coeffs = vars_to_solve_iter.clone().map(|i| eq[i]).collect();

//   let vars_to_solve_iter = vars_to_solve_iter.enumerate();
//   let all_options = get_all_ranges(coeffs, current_ans);
//   if all_options.is_empty() {
//     if to_solve == 0 {
//       vec![solved_variables.clone()]
//     } else {
//       get_solutions(equations, solved_variables, to_solve - 1)
//     }
//   } else {
//     all_options
//       .into_iter()
//       .flat_map(|option| {
//         let mut new_solved = solved_variables.clone();
//         for (i, j) in vars_to_solve_iter.clone() {
//           new_solved[j] = Some(option[i] as u64);
//         }
//         if to_solve == 0 {
//           vec![new_solved]
//         } else {
//           get_solutions(equations, &new_solved, to_solve - 1)
//         }
//       })
//       .collect::<Vec<Vec<Option<u64>>>>()
//   }
// }

// fn get_all_ranges(coeffs: Vec<Frac>, target_num: Frac) -> Vec<Vec<u32>> {
//   if coeffs.is_empty() {
//     vec![]
//   } else if coeffs.len() == 1 {
//     let max_i = target_num / coeffs[0];
//     if !max_i.is_whole() || max_i.is_negative() {
//       vec![]
//     } else {
//       vec![vec![max_i.floor() as u32]]
//     }
//   } else {
//     let last_coeff = coeffs[coeffs.len()-1];
//     let max_i = (target_num / last_coeff).floor();
//     if max_i < 0 {
//       vec![]
//     } else {
//       (0..=max_i)
//         .flat_map(|i| {
//           let new_target = target_num - last_coeff * i;
//           let new_coeffs: Vec<Frac> = coeffs.clone().into_iter().take(coeffs.len()-1).collect();
//           get_all_ranges(new_coeffs, new_target)
//             .into_iter()
//             .map(|mut new_range| {
//               new_range.push(i as u32);
//               new_range
//             })
//             .collect::<Vec<Vec<u32>>>()
//         })
//         .collect()
//     }
//   }
// }

fn parse_line(line: &str) -> (Vec<u32>, Vec<Frac>) {
  let (_, rest) = line
    .split_once("] (")
    .unwrap();
  let (buttons_str, joltage_str) = rest.split_once(") {").unwrap();
  let buttons: Vec<u32> = buttons_str
    .split(") (")
    .map(|button| button
      .split(",")
      .map(|b| b.parse::<u32>().unwrap())
      .fold(0u32, |acc, b| acc | 1 << b))
    .collect();

  let joltage = joltage_str
    .strip_suffix('}')
    .unwrap()
    .split(',')
    .map(|c| Frac::new(c.parse::<i16>().unwrap(), 1))
    .collect::<Vec<Frac>>();

  (buttons, joltage)
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
