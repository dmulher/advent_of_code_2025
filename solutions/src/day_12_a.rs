use std::collections::HashSet;

extern crate test;

pub fn main(contents: String) -> u16 {
  do_something(contents)
}

fn do_something(contents: String) -> u16 {
  // Generally, we want to map them all to binary numbers, and we will do... Something
  let (present_blocks, map_block) = contents.rsplit_once("\n\n").unwrap();
  let presents: Vec<HashSet<u16>> = present_blocks
    .split("\n\n")
    .map(|block| {
      let new_present: u16 = block
        .lines()
        .skip(1)
        .fold(0, |acc, line| {
          (acc << 3) + line.chars().fold(0, |acc2, c| (acc2 << 1) + (if c == '#' {1} else {0}))
        }) << 4;
      present_possibilities(new_present + new_present.count_ones() as u16)
    })
    .collect();
  for present in presents.iter() {
    print_shape(*present.iter().next().unwrap());
  }
  map_block
    .lines()
    .filter(|line| {
      let (size, parts) = line.split_once(':').unwrap();
      let size = size.split_once('x').unwrap();
      let (x, y) = (size.0.parse::<u8>().unwrap(), size.1.parse::<u8>().unwrap());
      let targets: Vec<u8> = parts.split_whitespace().map(|p| p.parse::<u8>().unwrap()).collect();

      let total_bits_needed = (0..targets.len()).fold(0u16, |acc, pi| acc + (presents[pi].iter().next().unwrap() % 7) * targets[pi] as u16);
      println!("Total bits = {}", total_bits_needed);
      if total_bits_needed > (x as u16 * y as u16) {
        false
      } else {
        let map = vec![0u64; y as usize];
        does_work(&presents, &targets, &map, y, x, 0, 0)
      }
    })
    .count() as u16
}

fn does_work(presents: &Vec<HashSet<u16>>, targets: &Vec<u8>, map: &Vec<u64>, height: u8, width: u8, present_i: usize, present_no: usize) -> bool {
  let mut present_i = present_i;
  let mut present_no = present_no;
  while present_no as u8 >= targets[present_i] {
    if present_i + 1 == presents.len() {
      println!("IT WORKED!");
      print_map(map, width);
      return true;
    }
    present_i = present_i + 1;
    present_no = 0;
  }
  let present_configs = &presents[present_i];
  (0..(height as usize - 2))
    .any(|yi| {
      (0..(width-2))
        .any(|xi| {
          let x_shift = width - (xi + 3);
          present_configs.iter().any(|pres| {
            let pres_top = (pres >> 10) & 7;
            let pres_mid = (pres >> 7) & 7;
            let pres_bot = (pres >> 4) & 7;
            let top_works = (((map[yi] >> x_shift) & 7) as u16 & pres_top) == 0;
            let mid_works = (((map[yi+1] >> x_shift) & 7) as u16 & pres_mid) == 0;
            let bot_works = (((map[yi+2] >> x_shift) & 7) as u16 & pres_bot) == 0;
            if !top_works || !mid_works || !bot_works {
              // println!("Didn't work....");
            }
            if top_works && mid_works && bot_works {
              let mut new_map = map.clone();
              new_map[yi] = new_map[yi] ^ ((pres_top as u64) << x_shift);
              new_map[yi+1] = new_map[yi+1] ^ ((pres_mid as u64) << x_shift);
              new_map[yi+2] = new_map[yi+2] ^ ((pres_bot as u64) << x_shift);
              // println!("Using present {}", pres);
              // print_shape(*pres);
              // println!("From:");
              // print_map(map, width);
              // println!("To:");
              // print_map(&new_map, width);
              does_work(presents, targets, &new_map, height, width, present_i, present_no + 1)
            } else {
              false
            }
          })
        })
    })
}

fn print_map(map: &Vec<u64>, width: u8) -> () {
  for line in map.iter() {
    for i in 0..width {
      print!("{}", line >> (width - (i + 1)) & 1);
    }
    println!("");
  }
}

fn print_shape(shape: u16) -> () {
  println!();
  println!("{}{}{}", 1 & (shape >> 12), 1 & (shape >> 11), 1 & (shape >> 10));
  println!("{}{}{}", 1 & (shape >> 9), 1 & (shape >> 8), 1 & (shape >> 7));
  println!("{}{}{}", 1 & (shape >> 6), 1 & (shape >> 5), 1 & (shape >> 4));
}

fn present_possibilities(start_shape: u16) -> HashSet<u16> {
  let base = start_shape;
  let c_90 = ((base >> 12) & 1) << 10 | ((base >> 11) & 1) << 7 | ((base >> 10) & 1) << 4 | ((base >> 9) & 1) << 11 | (base & 1 << 8) | ((base >> 7) & 1) << 5 | ((base >> 6) & 1) << 12 | ((base >> 5) & 1) << 9 | ((base >> 4) & 1) << 6;
  let c_180 = ((base >> 12) & 1) << 4 | ((base >> 11) & 1) << 5 | ((base >> 10) & 1) << 6 | ((base >> 9) & 1) << 7 | (base & 1 << 8) | ((base >> 7) & 1) << 9 | ((base >> 6) & 1) << 10 | ((base >> 5) & 1) << 11 | ((base >> 4) & 1) << 12;
  let c_270 = ((base >> 12) & 1) << 6 | ((base >> 11) & 1) << 9 | ((base >> 10) & 1) << 12 | ((base >> 9) & 1) << 5 | (base & 1 << 8) | ((base >> 7) & 1) << 11 | ((base >> 6) & 1) << 4 | ((base >> 5) & 1) << 7 | ((base >> 4) & 1) << 10;
  let flip = ((base >> 12) & 1) << 6 | ((base >> 11) & 1) << 5 | ((base >> 10) & 1) << 4 | (base & (7 << 7)) | ((base >> 6) & 1) << 12 | ((base >> 5) & 1) << 11 | ((base >> 4) & 1) << 10;
  let f_90 = ((flip >> 12) & 1) << 10 | ((flip >> 11) & 1) << 7 | ((flip >> 10) & 1) << 4 | ((flip >> 9) & 1) << 11 | (flip & 1 << 8) | ((flip >> 7) & 1) << 5 | ((flip >> 6) & 1) << 12 | ((flip >> 5) & 1) << 9 | ((flip >> 4) & 1) << 6;
  let f_180 = ((flip >> 12) & 1) << 4 | ((flip >> 11) & 1) << 5 | ((flip >> 10) & 1) << 6 | ((flip >> 9) & 1) << 7 | (flip & 1 << 8) | ((flip >> 7) & 1) << 9 | ((flip >> 6) & 1) << 10 | ((flip >> 5) & 1) << 11 | ((flip >> 4) & 1) << 12;
  let f_270 = ((flip >> 12) & 1) << 6 | ((flip >> 11) & 1) << 9 | ((flip >> 10) & 1) << 12 | ((flip >> 9) & 1) << 5 | (flip & 1 << 8) | ((flip >> 7) & 1) << 11 | ((flip >> 6) & 1) << 4 | ((flip >> 5) & 1) << 7 | ((flip >> 4) & 1) << 10;
  HashSet::from([base, c_90, c_180, c_270, flip, f_90, f_180, f_270])
}

#[cfg(test)]
mod tests {
  use super::*;
  // use test::Bencher;
  // use utils::read_file_to_string;

  const DAY: u8 = 12;
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_12_a() {
    const EXAMPLE_ANSWER: Option<u16> = Some(2);
    const ANSWER: Option<u16> = None;
    match utils::run_method::<u16>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  // #[bench]
  // fn bench_day_12_a(b: &mut Bencher) {
  //   let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
  //   b.iter(|| main(input.clone()));
  // }
}
