use aoc_2024::day1;

fn main() {
  day1();
}

fn spacer() {
  println!("--------------------------------------------------")
}

fn day1() {
  spacer();

  let input = &day1::parse_input_file() as &str;

  let total_dist = day1::part1(input);
  let similarity_score = day1::part2(input);

  println!("[Day01::Part1] Total Distance => {}", total_dist);
  println!("[Day01::Part2] Similarity Score => {}", similarity_score);
}
