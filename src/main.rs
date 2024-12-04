use aoc_2024::day1;
use aoc_2024::day2;
use aoc_2024::day3;
use aoc_2024::util::io;

fn main() {
  day3();
  day2();
  day1();
}

fn day1() {
  io::spacer();

  let day: u8 = 1;
  let (_test_input, input) = io::get_day_inputs(day);

  let total_dist = day1::part1(&input);
  let similarity_score = day1::part2(&input);

  println!("[Day01::Part1] Total Distance => {}", total_dist);
  println!("[Day01::Part2] Similarity Score => {}", similarity_score);

  day1::benchmarks::run(&input, io::Env::Run);
  day1::benchmarks::run(&_test_input, io::Env::Test);
}

fn day2() {
  io::spacer();

  let day: u8 = 2;
  let (_test_input, input) = io::get_day_inputs(day);

  let safe_reports_count = day2::part1(&input);
  let dampened_safe_reports_count = day2::part2(&input);

  println!("[Day02::Part1] Safe Reports Count => {safe_reports_count}");
  println!("[Day02::Part2] Dampened Safe Reports Count => {dampened_safe_reports_count}");

  day2::benchmarks::run(&input, io::Env::Run);
  day2::benchmarks::run(&_test_input, io::Env::Test);
}

fn day3() {
  io::spacer();

  let day: u8 = 3;
  let (_test_input, input) = io::get_day_inputs(day);

  let part1_result = day3::part1(&input);
  let part2_result = day3::part2(&input);

  println!("[Day03::Part1] Part 1 Result => {part1_result}");
  println!("[Day03::Part2] Part 2 Result => {part2_result}");

  day3::benchmarks::run(&input, io::Env::Run);
  day3::benchmarks::run(&_test_input, io::Env::Test);
}
