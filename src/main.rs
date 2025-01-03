use aoc_2024::day1;
use aoc_2024::day10;
use aoc_2024::day11;
use aoc_2024::day12;
use aoc_2024::day13;
use aoc_2024::day14;
use aoc_2024::day15;
use aoc_2024::day2;
use aoc_2024::day22;
use aoc_2024::day23;
use aoc_2024::day24;
use aoc_2024::day25;
use aoc_2024::day3;
use aoc_2024::day4;
use aoc_2024::day5;
use aoc_2024::day6;
use aoc_2024::day7;
use aoc_2024::day8;
use aoc_2024::day9;
use aoc_2024::util::io;

fn main() {
  _day25();
  _day24();
  _day23();
  _day22();
  _day15();
  _day14();
  _day13();
  _day12();
  _day11();
  _day10();
  _day9();
  _day8();
  _day7();
  _day6();
  _day5();
  _day4();
  _day3();
  _day2();
  _day1();
}

fn day_init(day: u8) -> (String, String) {
  io::spacer();
  io::get_day_inputs(day)
}

fn _day1() {
  let day: u8 = 1;
  let (_test_input, input) = day_init(day);

  let total_dist = day1::part1(&input);
  let similarity_score = day1::part2(&input);

  println!("[Day{day:02}::Part1] Total Distance => {}", total_dist);
  println!("[Day{day:02}::Part2] Similarity Score => {}", similarity_score);

  day1::benchmarks::run(&input, io::Env::Run);
  day1::benchmarks::run(&_test_input, io::Env::Test);
}

fn _day2() {
  let day: u8 = 2;
  let (_test_input, input) = day_init(day);

  let safe_reports_count = day2::part1(&input);
  let dampened_safe_reports_count = day2::part2(&input);

  println!("[Day{day:02}::Part1] Safe Reports Count => {safe_reports_count}");
  println!("[Day{day:02}::Part2] Dampened Safe Reports Count => {dampened_safe_reports_count}");

  day2::benchmarks::run(&input, io::Env::Run);
  day2::benchmarks::run(&_test_input, io::Env::Test);
}

fn _day3() {
  let day: u8 = 3;
  let (_test_input, input) = day_init(day);

  let part1_result = day3::part1(&input);
  let part2_result = day3::part2(&input);

  println!("[Day{day:02}::Part1] Part 1 Result => {part1_result}");
  println!("[Day{day:02}::Part2] Part 2 Result => {part2_result}");

  day3::benchmarks::run(&input, io::Env::Run);
  day3::benchmarks::run(&_test_input, io::Env::Test);
}

fn _day4() {
  let day: u8 = 4;
  let (_test_input, input) = day_init(day);

  let part1_result = day4::part1(&input);
  let part2_result = day4::part2(&input);

  println!("[Day{day:02}::Part1] Part 1 Result => {part1_result}");
  println!("[Day{day:02}::Part2] Part 2 Result => {part2_result}");

  day4::benchmarks::run(&input, io::Env::Run);
  day4::benchmarks::run(&_test_input, io::Env::Test);
}

fn _day5() {
  let day: u8 = 5;
  let (_test_input, input) = day_init(day);

  let part1_result = day5::part1(&input);
  let part2_result = day5::part2(&input);

  println!("[Day{day:02}::Part1] Part 1 Result => {part1_result}");
  println!("[Day{day:02}::Part2] Part 2 Result => {part2_result}");

  day5::benchmarks::run(&input, io::Env::Run);
  day5::benchmarks::run(&_test_input, io::Env::Test);
}

fn _day6() {
  let day: u8 = 6;
  let (_test_input, input) = day_init(day);

  let part1_result = day6::part1(&input);
  let part2_result = day6::part2(&input);

  println!("[Day{day:02}::Part1] Part 1 Result => {part1_result}");
  println!("[Day{day:02}::Part2] Part 2 Result => {part2_result}");

  day6::benchmarks::run(&input, io::Env::Run);
  day6::benchmarks::run(&_test_input, io::Env::Test);
}

fn _day7() {
  let day: u8 = 7;
  let (_test_input, input) = day_init(day);

  let part1_result = day7::part1(&input);
  let part2_result = day7::part2(&input);

  println!("[Day{day:02}::Part1] Part 1 Result => {part1_result}");
  println!("[Day{day:02}::Part2] Part 2 Result => {part2_result}");

  day7::benchmarks::run(&input, io::Env::Run);
  day7::benchmarks::run(&_test_input, io::Env::Test);
}

fn _day8() {
  let day: u8 = 8;
  let (_test_input, input) = day_init(day);

  let part1_result = day8::part1(&input);
  let part2_result = day8::part2(&input);

  println!("[Day{day:02}::Part1] Part 1 Result => {part1_result}");
  println!("[Day{day:02}::Part2] Part 2 Result => {part2_result}");

  day8::benchmarks::run(&input, io::Env::Run);
  day8::benchmarks::run(&_test_input, io::Env::Test);
}

fn _day9() {
  let day: u8 = 9;
  let (_test_input, input) = day_init(day);

  let part1_result = day9::part1(&input);
  let part2_result = day9::part2(&input);

  println!("[Day{day:02}::Part1] Part 1 Result => {part1_result}");
  println!("[Day{day:02}::Part2] Part 2 Result => {part2_result}");

  day9::benchmarks::run(&_test_input, io::Env::Test);
  day9::benchmarks::run(&input, io::Env::Run);
}

fn _day10() {
  let day: u8 = 10;
  let (_test_input, input) = day_init(day);

  let part1_result = day10::part1(&input);
  let part2_result = day10::part2(&input);

  println!("[Day{day:02}::Part1] Part 1 Result => {part1_result}");
  println!("[Day{day:02}::Part2] Part 2 Result => {part2_result}");

  day10::benchmarks::run(&_test_input, io::Env::Test);
  day10::benchmarks::run(&input, io::Env::Run);
}

fn _day11() {
  let day: u8 = 11;
  let (_test_input, input) = day_init(day);

  let part1_result = day11::part1(&input);
  let part2_result = day11::part2(&input);

  println!("[Day{day:02}::Part1] Part 1 Result => {part1_result}");
  println!("[Day{day:02}::Part2] Part 2 Result => {part2_result}");

  day11::benchmarks::run(&_test_input, io::Env::Test);
  day11::benchmarks::run(&input, io::Env::Run);
}

fn _day12() {
  let day: u8 = 12;
  let (_test_input, input) = day_init(day);

  // let part1_result = day12::part1(&_test_input);
  let part1_result = day12::part1(&input);
  // let part2_result = day12::part2(&input);

  println!("[Day{day:02}::Part1] Part 1 Result => {part1_result}");
  // println!("[Day{day:02}::Part2] Part 2 Result => {part2_result}");

  // day12::benchmarks::run(&_test_input, io::Env::Test);
  // day12::benchmarks::run(&input, io::Env::Run);
}

fn _day13() {
  let day: u8 = 13;
  let (_test_input, input) = day_init(day);

  let part1_result = day13::part1(&input);
  let part2_result = day13::part2(&input);

  println!("[Day{day:02}::Part1] Part 1 Result => {part1_result}");
  println!("[Day{day:02}::Part2] Part 2 Result => {part2_result}");

  day13::benchmarks::run(&_test_input, io::Env::Test);
  day13::benchmarks::run(&input, io::Env::Run);
}

fn _day14() {
  let day: u8 = 14;
  let (_test_input, input) = day_init(day);

  let part1_result = day14::part1(&input);
  let part2_result = day14::part2(&input);

  println!("[Day{day:02}::Part1] Part 1 Result => {part1_result}");
  println!("[Day{day:02}::Part2] Part 2 Result => {part2_result}");

  day14::benchmarks::run(&_test_input, day14::Matrix { x: 11, y: 7 }, io::Env::Test);
  day14::benchmarks::run(&input, day14::Matrix { x: 101, y: 103 }, io::Env::Run);
}

fn _day15() {
  let day: u8 = 15;
  let (_test_input, input) = day_init(day);

  let part1_result = day15::part1(&input);
  // let part2_result = day15::part2(&input);

  println!("[Day{day:02}::Part1] Part 1 Result => {part1_result}");
  // println!("[Day{day:02}::Part2] Part 2 Result => {part2_result}");

  day15::benchmarks::run(&_test_input, io::Env::Test);
  day15::benchmarks::run(&input, io::Env::Run);
}

fn _day22() {
  let day: u8 = 22;
  let (_test_input, input) = day_init(day);

  let part1_result = day22::part1(&input);
  let part2_result = day22::part2(&input);

  println!("[Day{day:02}::Part1] Part 1 Result => {part1_result}");
  println!("[Day{day:02}::Part2] Part 2 Result => {part2_result}");

  day22::benchmarks::run(&_test_input, io::Env::Test);
  day22::benchmarks::run(&input, io::Env::Run);
}

fn _day23() {
  let day: u8 = 23;
  let (_test_input, input) = day_init(day);

  let part1_result = day23::part1(&input);
  let part2_result = day23::part2(&input);

  println!("[Day{day:02}::Part1] Part 1 Result => {part1_result}");
  println!("[Day{day:02}::Part2] Part 2 Result => {part2_result}");

  day23::benchmarks::run(&_test_input, io::Env::Test);
  day23::benchmarks::run(&input, io::Env::Run);
}

fn _day24() {
  let day: u8 = 24;
  let (_test_input, input) = day_init(day);

  let part1_result = day24::part1(&input);

  println!("[Day{day:02}::Part1] Part 1 Result => {part1_result}");

  day24::benchmarks::run(&_test_input, io::Env::Test);
  day24::benchmarks::run(&input, io::Env::Run);
}

fn _day25() {
  let day: u8 = 25;
  let (_test_input, input) = day_init(day);

  let part1_result = day25::part1(&input);

  println!("[Day{day:02}::Part1] Part 1 Result => {part1_result}");

  day25::benchmarks::run(&_test_input, io::Env::Test);
  day25::benchmarks::run(&input, io::Env::Run);
}
