pub mod benchmarks;

fn to_pin_heights<'a, I>(input: I) -> [u8; 5]
where
  I: Iterator<Item = &'a str>,
{
  let mut pin_heights: [u8; 5] = [0; 5];
  input.for_each(|l| {
    l.chars().enumerate().for_each(|(i, c)| {
      if c == '#' {
        pin_heights[i] += 1;
      }
    });
  });
  return pin_heights;
}

fn parse_lock(input: &str) -> [u8; 5] {
  to_pin_heights(input.lines().skip(1))
}

fn parse_key(input: &str) -> [u8; 5] {
  to_pin_heights(input.lines().rev().skip(1))
}

fn parse_input(input: &str) -> (Vec<[u8; 5]>, Vec<[u8; 5]>) {
  let mut locks: Vec<[u8; 5]> = vec![];
  let mut keys: Vec<[u8; 5]> = vec![];

  input.trim().split("\n\n").for_each(|s| {
    if s.starts_with("#####") {
      locks.push(parse_lock(s));
    } else {
      keys.push(parse_key(s));
    }
  });

  (keys, locks)
}

fn count_non_overlapping(keys: Vec<[u8; 5]>, locks: Vec<[u8; 5]>) -> usize {
  keys.into_iter().fold(0_usize, |acc, k| {
    let non_overlapping = locks.iter().fold(0_usize, |acc, l| {
      for i in 0..5 {
        let overlaps = k[i] + l[i] > 5;
        if overlaps {
          return acc;
        }
      }
      acc + 1
    });
    acc + non_overlapping
  })
}

pub fn part1(input: &str) -> usize {
  let (keys, locks) = parse_input(input);
  let non_overlapping_count = count_non_overlapping(keys, locks);
  non_overlapping_count
}

pub fn part2(input: &str) -> usize {
  let (keys, locks) = parse_input(input);
  keys.len() + locks.len()
}
