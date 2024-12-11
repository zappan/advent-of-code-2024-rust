pub mod benchmarks;
pub mod via_hashmap;
pub mod via_vec;

pub fn part1(input: &str) -> usize {
  via_vec::part1(input)
}

pub fn part2(input: &str) -> usize {
  via_vec::part2(input)
}
