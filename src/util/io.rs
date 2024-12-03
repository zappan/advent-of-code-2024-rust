use std::fmt;
use std::fs;

#[derive(Debug)]
pub enum Env {
  Run,
  Test,
}

impl fmt::Display for Env {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str(&format!("{:?}", *self))
  }
}

pub fn spacer() {
  println!("====================================================");
}

pub fn bench_spacer(env: Env) {
  println!("----------------- Benchmarks {:<4} ------------------", env.to_string());
}

pub fn get_day_inputs(day: u8) -> (String, String) {
  let _test_input = read_test_input_file(day).to_string();
  let input = read_input_file(day).to_string();
  (_test_input, input)
}

fn get_input_file_name(day: u8, env: Env) -> String {
  match env {
    Env::Test => format!("input/test/day{:02}.txt", day),
    Env::Run => format!("input/day{:02}.txt", day),
  }
}

fn read_file(file: String) -> String {
  let file_contents = fs::read_to_string(file).unwrap();
  return file_contents;
}

fn read_input_file(day: u8) -> String {
  read_file(get_input_file_name(day, Env::Run))
}

fn read_test_input_file(day: u8) -> String {
  read_file(get_input_file_name(day, Env::Test))
}
