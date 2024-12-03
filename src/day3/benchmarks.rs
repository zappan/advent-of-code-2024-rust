use crate::util::io::{bench_spacer, Env};

use super::*;

#[derive(Debug)]
enum BenchImpl {
  Part1Regex,
  Part1Walkthrough,
  Part2Regex,
  Part2Walkthrough,
}

fn run_benchmark(input: &str, fn_impl: BenchImpl) {
  let now = std::time::Instant::now();
  let result = match fn_impl {
    BenchImpl::Part1Regex => {
      let parsed_input = parse_input_regex(input);
      let result = calculate_sum(&parsed_input);
      result
    }
    BenchImpl::Part1Walkthrough => {
      let parsed_input = parse_input_walkthrough(input);
      let result = calculate_sum(&parsed_input);
      result
    }
    BenchImpl::Part2Regex => {
      let preprocessed_input = preprocessor::preprocess(input);
      let parsed_input = parse_input_regex(&preprocessed_input);
      let result = calculate_sum(&parsed_input);
      result
    }
    BenchImpl::Part2Walkthrough => {
      let preprocessed_input = preprocessor::preprocess(input);
      let parsed_input = parse_input_regex(&preprocessed_input);
      let result = calculate_sum(&parsed_input);
      result
    }
  };
  let elapsed = now.elapsed();
  println!("[{fn_impl:#?}] Elapsed {elapsed:.2?}; Result: {result}");
}

pub fn run(input: &str, env: Env) {
  bench_spacer(env);
  run_benchmark(input, BenchImpl::Part1Walkthrough);
  run_benchmark(input, BenchImpl::Part1Regex);
  run_benchmark(input, BenchImpl::Part2Walkthrough);
  run_benchmark(input, BenchImpl::Part2Regex);
}
