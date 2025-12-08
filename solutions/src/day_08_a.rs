use std::collections::BinaryHeap;

extern crate test;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Coords {
  x: i64,
  y: i64,
  z: i64
}

impl Coords {
  pub fn distance(&self, other: Self) -> u128 {
    (other.x - self.x).pow(2) as u128 + (other.y - self.y).pow(2) as u128 + (other.z - self.z).pow(2) as u128
  }
}

#[derive(Debug, PartialEq, Eq)]
struct Distance {
  origin: usize,
  target: usize,
  distance: u128,
}

impl PartialOrd for Distance {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    other.distance.partial_cmp(&self.distance)
  }
}

impl Ord for Distance {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.distance.cmp(&other.distance)
  }
}

// #[derive(Debug, PartialEq, Eq, Clone)]
// enum Link {
//   LinkedTo(usize),
//   Link(Vec<usize>)
// }

// impl Link {
//   fn push(&mut self, item: usize) -> () {
//     match self {
//       Link::LinkedTo(_) => panic!("Can't push to me"),
//       Link::Link(l) => l.push(item),
//     }
//   }
// }

pub fn main(contents: String) -> u32 {
  circuit_schenanigans(contents)
}

fn circuit_schenanigans(contents: String) -> u32 {
  let coords = contents
    .lines()
    .map(|line| {
      let mut splitted = line.splitn(3, ',').map(|coord| coord.parse::<i64>().unwrap());
      let x = splitted.next().unwrap();
      let y = splitted.next().unwrap();
      let z = splitted.next().unwrap();
      Coords{ x, y, z }
    })
    .collect::<Vec<Coords>>();
  let mut distances = BinaryHeap::<Distance>::new();
  for i in 0..coords.len()-1 {
    for j in i+1..coords.len() {
      let i_coord = coords[i];
      let j_coord = coords[j];
      distances.push(Distance {
        origin: i,
        target: j,
        distance: i_coord.distance(j_coord)
      });
    }
  }
  let mut links: Vec<Vec<usize>> = vec![];
  let mut abort: u16 = if coords.len() == 20 {10} else {1000};
  while let Some(distance) = distances.pop() {
    if abort == 0 {
      break;
    }
    abort -= 1;
    let i = links.iter().enumerate().filter(|l| l.1.contains(&distance.origin)).map(|(c, _)| c).next();
    let j = links.iter().enumerate().filter(|l| l.1.contains(&distance.target)).map(|(c, _)| c).next();
    match (i, j) {
      (None, None) => links.push(vec![distance.origin, distance.target]),
      (Some(i), None) => links[i].push(distance.target),
      (None, Some(j)) => links[j].push(distance.origin),
      (Some(i), Some(j)) => {
        if i != j {
          let mut drain = links.remove(j);
          if j > i {
            links[i].append(&mut drain)
          } else {
            links[i-1].append(&mut drain)
          }
        }
      },
    }
  }
  links.sort_by(|a, b| b.len().cmp(&a.len()));
  println!("{:?}", links);
  let mut lengths: Vec<usize> = links
    .into_iter()
    .map(|d| d.len())
    .collect();
  lengths.sort_by(|a, b| b.cmp(a));
  (lengths.get(0).unwrap_or(&1) * lengths.get(1).unwrap_or(&1) * lengths.get(2).unwrap_or(&1)) as u32
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 8;
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_08_a() {
    const EXAMPLE_ANSWER: Option<u32> = Some(40);
    const ANSWER: Option<u32> = Some(46398);
    match utils::run_method::<u32>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_08_a(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
