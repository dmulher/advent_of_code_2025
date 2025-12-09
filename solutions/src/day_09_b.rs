extern crate test;

#[derive(Debug)]
struct BBox {
  top: u32,
  bot: u32,
  left: u32,
  right: u32,
}

fn bbox_volume(bbox: &BBox) -> u64 {
  (bbox.bot - bbox.top + 1) as u64 * (bbox.right - bbox.left + 1) as u64
}

fn bbox_overlap(b1: &BBox, b2: &BBox) -> u64 {
  bbox_volume(&BBox {
    top: b1.top.max(b2.top),
    bot: b1.bot.min(b2.bot),
    left: b1.left.max(b2.left),
    right: b1.right.min(b2.right)
  })
}

#[derive(Debug)]
enum Line {
  Horizontal(u32, u32, u32),
  Vertical(u32, u32, u32)
}

fn line_volume(l: &Line) -> u64 {
  match l {
    Line::Horizontal(_, x1, x2) => (x2-x1) as u64,
    Line::Vertical(_, y1, y2) => (y2-y1) as u64,
  }
}

fn line_overlap(l: &Line, bbox: &BBox) -> u64 {
  match l {
    Line::Horizontal(_, x1, x2) => if does_line_intersect_box(l, bbox) { (x2.min(&bbox.right) - x1.max(&bbox.left)) as u64 } else { 0 },
    Line::Vertical(_, y1, y2) => if does_line_intersect_box(l, bbox) { (y2.min(&bbox.bot) - y1.max(&bbox.top)) as u64 } else { 0 },
  }
}

fn join_line_and_line(l1: &Line, l2: &Line) -> BBox {
  match (l1, l2) {
    (Line::Horizontal(y1, x1, x2), Line::Horizontal(y2, x3, x4)) => BBox {
      top: *y1.min(y2),
      bot: *y1.max(y2),
      left: *x1.min(x3),
      right: *x2.max(x4),
    },
    (Line::Horizontal(y, x1, x2), Line::Vertical(x, y1, y2)) |
    (Line::Vertical(x, y1, y2), Line::Horizontal(y, x1, x2)) => BBox {
      top: *y.min(y1),
      bot: *y.max(y2),
      left: *x1.min(x),
      right: *x2.max(x),
    },
    (Line::Vertical(x1, y1, y2), Line::Vertical(x2, y3, y4)) => BBox {
      top: *y1.min(y3),
      bot: *y2.max(y4),
      left: *x1.min(x2),
      right: *x1.max(x2),
    },
  }
}

fn join_line_and_box(line: &Line, bbox: &BBox) -> BBox {
  match line {
    Line::Horizontal(y, x1, x2) => BBox{
      top: bbox.top.min(*y),
      bot: bbox.bot.max(*y),
      left: bbox.left.min(*x1),
      right: bbox.right.max(*x2)
    },
    Line::Vertical(x, y1, y2) => BBox{
      top: bbox.top.min(*y1),
      bot: bbox.bot.max(*y2),
      left: bbox.left.min(*x),
      right: bbox.right.max(*x)
    },
  }

}

#[derive(Debug)]
enum AABBNode {
  Leaf(Line),
  Branch(BBox, Box<AABBNode>, Box<AABBNode>)
}

// TODO: Finish implementation
impl AABBNode {
  pub fn merge_bbox(&self, line: &Line) -> BBox {
    match self {
      AABBNode::Branch(bbox, _, _) => {
        join_line_and_box(line, bbox)
      },
      AABBNode::Leaf(leaf) => {
        join_line_and_line(line, leaf)
      },
    }
  }

  pub fn overlap(&self, b2: &BBox) -> u64 {
    match self {
      AABBNode::Leaf(l) => line_overlap(l, b2),
      AABBNode::Branch(b1, _, _) => bbox_overlap(b1, b2)
    }
  }

  pub fn volume(&self) -> u64 {
    match self {
      AABBNode::Leaf(l) => line_volume(l),
      AABBNode::Branch(bbox, _, _) => bbox_volume(bbox)
    }
  }

  pub fn add(self, new_line: Line) -> AABBNode {
    let new_base_box = self.merge_bbox(&new_line);
    match self {
      AABBNode::Leaf(_) => {
        AABBNode::Branch(new_base_box, Box::new(self), Box::new(Self::Leaf(new_line)))
      },
      AABBNode::Branch(bbox, lb, rb) => {
        let left_merged_box = lb.merge_bbox(&new_line);
        let right_merged_box = rb.merge_bbox(&new_line);

        let base_volume = bbox_volume(&new_base_box);
        let left_volume = bbox_volume(&left_merged_box);
        let right_volume = bbox_volume(&right_merged_box);

        let left_overlap = lb.overlap(&left_merged_box);
        let right_overlap = rb.overlap(&right_merged_box);

        let base_volume_change = base_volume - bbox_volume(&bbox);
        let base_cost = base_volume;
        let left_cost = base_volume_change + (left_volume - lb.volume()) + left_overlap;
        let right_cost = base_volume_change + (right_volume - rb.volume()) + right_overlap;

        if base_cost < left_cost && base_cost < right_cost {
          AABBNode::Branch(new_base_box, Box::new(AABBNode::Branch(bbox, lb, rb)), Box::new(AABBNode::Leaf(new_line)))
        } else if left_cost < right_cost {
          AABBNode::Branch(new_base_box, Box::new(lb.add(new_line)), rb)
        } else {
          AABBNode::Branch(new_base_box, lb, Box::new(rb.add(new_line)))
        }
      }
    }
  }

  pub fn intersects(&self, b1: &BBox) -> bool {
    match self {
      AABBNode::Leaf(l) => does_line_intersect_box(l, b1),
      AABBNode::Branch(b2, lb, rb) => does_box_intersect_box(b1, b2) && (lb.intersects(b1) || rb.intersects(b1))
    }
  }
}

fn does_line_intersect_box(l: &Line, bbox: &BBox) -> bool {
  match l {
    Line::Horizontal(y, x1, x2) => {
      if y > &bbox.top && y < &bbox.bot {
        if x1 < &bbox.right && x2 > &bbox.left {
          return true;
        }
      }
    },
    Line::Vertical(x, y1, y2) => {
      if x > &bbox.left && x < &bbox.right {
        if y1 < &bbox.bot && y2 > &bbox.top {
          return true;
        }
      }
    }
  }
  false
}

fn does_box_intersect_box(b1: &BBox, b2: &BBox) -> bool {
  b1.right > b2.left && b1.left < b2.right && b1.bot > b2.top && b1.top < b2.bot
}

pub fn main(contents: String) -> u64 {
  largest_rectangle(contents)
}

fn largest_rectangle(contents: String) -> u64 {
  let mut coords_iter = contents
    .lines()
    .map(|l| {
      let (a, b) = l.split_once(',').unwrap();
      (b.parse::<u32>().unwrap(), a.parse::<u32>().unwrap())  // y, x
    });
  let coords = coords_iter.clone().collect::<Vec<(u32, u32)>>();

  let last_coord = coords[coords.len()-1];
  let first_line = make_line(&coords_iter.next().unwrap(), &last_coord);
  let (_, tree) = coords_iter
    .fold((last_coord, AABBNode::Leaf(first_line)), |(last_coord, tree), coord| {
      (coord, tree.add(make_line(&coord, &last_coord)))
    });
  (0..coords.len()).fold(0, |largest, i| {
    (0..coords.len() - 1).fold(largest, |acc, j| {
      let new_box = BBox {
        top: coords[i].0.min(coords[j].0),
        bot: coords[i].0.max(coords[j].0),
        left: coords[i].1.min(coords[j].1),
        right: coords[i].1.max(coords[j].1)
      };
      let volume = bbox_volume(&new_box);
      if volume > acc && !tree.intersects(&new_box) {
        volume
      } else {
        acc
      }
    })
  })
}

fn make_line(coord: &(u32, u32), last_coord: &(u32, u32)) -> Line {
  if coord.0 == last_coord.0 {
    Line::Horizontal(coord.0, last_coord.1.min(coord.1), last_coord.1.max(coord.1))
  } else {
    Line::Vertical(coord.1, last_coord.0.min(coord.0), last_coord.0.max(coord.0))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 9;
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_09_b() {
    const EXAMPLE_ANSWER: Option<u64> = Some(24);
    const ANSWER: Option<u64> = Some(1560475800);
    match utils::run_method::<u64>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_09_b(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
