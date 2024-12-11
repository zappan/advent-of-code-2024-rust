pub mod benchmarks;
pub mod via_iterative;
pub mod via_memoized_tree_traversal;
pub mod via_tree_traversal;

pub fn part1(input: &str) -> usize {
  via_memoized_tree_traversal::part1(input)
}

pub fn part2(input: &str) -> usize {
  via_memoized_tree_traversal::part2(input)
}
