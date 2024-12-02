use crate::util::io::{bench_spacer, Env};

use super::*;

#[derive(Debug)]
enum BenchImpl {
  Part1Std,
  Part2Std,
}

fn run_benchmark(input: &str, fn_impl: BenchImpl) {
  let mut reports_data: Vec<Vec<u8>> = parse_input(input);
  // println!("parsed_input: {:?}", reports_data);

  let now = std::time::Instant::now();
  let result = match fn_impl {
    BenchImpl::Part1Std => count_safe_reports(&mut reports_data, validate_report_line),
    BenchImpl::Part2Std => count_safe_reports(&mut reports_data, validate_report_line_with_dampener),
  };
  let elapsed = now.elapsed();
  println!("[{fn_impl:#?}] Elapsed {elapsed:.2?}; Result: {result}");
}

pub fn run(input: &str, env: Env) {
  bench_spacer(env);
  run_benchmark(input, BenchImpl::Part1Std);
  run_benchmark(input, BenchImpl::Part2Std);
}
