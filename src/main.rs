use aoc_2024::day1;
use aoc_2024::day2;
use aoc_2024::util::io;

fn main() {
  day2();
  day1();
}

fn spacer() {
  println!("====================================================");
}

fn bench_spacer() {
  println!("-------------------- Benchmarks --------------------");
}

fn day1() {
  spacer();

  let day: u8 = 1;
  let _test_input = &io::read_test_input_file(day) as &str;
  let input = &io::read_input_file(day) as &str;

  let total_dist = day1::part1(input);
  let similarity_score = day1::part2(input);

  println!("[Day01::Part1] Total Distance => {}", total_dist);
  println!("[Day01::Part2] Similarity Score => {}", similarity_score);

  day1::benchmarks::run(input, bench_spacer);
  day1::benchmarks::run(_test_input, bench_spacer);
}

fn day2() {
  spacer();

  let day: u8 = 2;
  let _test_input = &io::read_test_input_file(day) as &str;
  let input = &io::read_input_file(day) as &str;

  let safe_reports_count = day2::part1(input);
  let dampened_safe_reports_count = day2::part2(input);

  println!("[Day02::Part1] Safe Reports Count => {safe_reports_count}");
  println!("[Day02::Part2] Dampened Safe Reports Count => {dampened_safe_reports_count}");

  day2::benchmarks::run(_test_input, bench_spacer);
  day2::benchmarks::run(input, bench_spacer);
}
