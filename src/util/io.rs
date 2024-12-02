use std::fs;

enum Env {
  Run,
  Test,
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

pub fn read_input_file(day: u8) -> String {
  read_file(get_input_file_name(day, Env::Run))
}

pub fn read_test_input_file(day: u8) -> String {
  read_file(get_input_file_name(day, Env::Test))
}
