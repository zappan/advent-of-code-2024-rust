use aoc_2024::day1;
use aoc_2024::day2;
use aoc_2024::day3;
use aoc_2024::day4;
use aoc_2024::util::io;

fn main() {
  day4();
  day3();
  day2();
  day1();
}

fn day_init(day: u8) -> (String, String) {
  io::spacer();
  io::get_day_inputs(day)
}

fn day1() {
  let day: u8 = 1;
  let (_test_input, input) = day_init(day);

  let total_dist = day1::part1(&input);
  let similarity_score = day1::part2(&input);

  println!("[Day{day:02}::Part1] Total Distance => {}", total_dist);
  println!("[Day{day:02}::Part2] Similarity Score => {}", similarity_score);

  day1::benchmarks::run(&input, io::Env::Run);
  day1::benchmarks::run(&_test_input, io::Env::Test);
}

fn day2() {
  let day: u8 = 2;
  let (_test_input, input) = day_init(day);

  let safe_reports_count = day2::part1(&input);
  let dampened_safe_reports_count = day2::part2(&input);

  println!("[Day{day:02}::Part1] Safe Reports Count => {safe_reports_count}");
  println!("[Day{day:02}::Part2] Dampened Safe Reports Count => {dampened_safe_reports_count}");

  day2::benchmarks::run(&input, io::Env::Run);
  day2::benchmarks::run(&_test_input, io::Env::Test);
}

fn day3() {
  let day: u8 = 3;
  let (_test_input, input) = day_init(day);

  let part1_result = day3::part1(&input);
  let part2_result = day3::part2(&input);

  println!("[Day{day:02}::Part1] Part 1 Result => {part1_result}");
  println!("[Day{day:02}::Part2] Part 2 Result => {part2_result}");

  day3::benchmarks::run(&input, io::Env::Run);
  day3::benchmarks::run(&_test_input, io::Env::Test);
}

fn day4() {
  let day: u8 = 4;
  let (_test_input, input) = day_init(day);

  let part1_result = day4::part1(&input);
  let part2_result = day4::part2(&input);

  println!("[Day{day:02}::Part1] Part 1 Result => {part1_result}");
  println!("[Day{day:02}::Part2] Part 2 Result => {part2_result}");

  day4::benchmarks::run(&input, io::Env::Run);
  day4::benchmarks::run(&_test_input, io::Env::Test);
}
