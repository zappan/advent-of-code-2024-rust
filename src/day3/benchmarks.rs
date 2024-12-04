use crate::util::io::{bench_spacer, Env};

use super::*;

#[derive(Debug)]
enum BenchImpl {
  Part1Walkthrough,
  Part1RegexParse,
  Part1RegexCapture,
  Part2Walkthrough,
  Part2RegexParse,
  Part2RegexCapture,
  Part2RegexPreprocessAndCapture,
}

fn run_benchmark(input: &str, fn_impl: BenchImpl) {
  let matching_expr = Regex::new(r"mul\((\d+)\,(\d+)\)").unwrap();

  let now = std::time::Instant::now();
  let result = match fn_impl {
    BenchImpl::Part1RegexParse => {
      let parsed_input = parse_input_regex(input, &matching_expr);
      let result = calculate_sum(&parsed_input, &matching_expr);
      result
    }
    BenchImpl::Part1RegexCapture => {
      let result = part1_regex_sum(&input);
      result
    }
    BenchImpl::Part1Walkthrough => {
      let parsed_input = parse_input_walkthrough(input);
      let result = calculate_sum(&parsed_input, &matching_expr);
      result
    }
    BenchImpl::Part2RegexParse => {
      let preprocessed_input = preprocessor::preprocess(input);
      let parsed_input = parse_input_regex(&preprocessed_input, &matching_expr);
      let result = calculate_sum(&parsed_input, &matching_expr);
      result
    }
    BenchImpl::Part2RegexPreprocessAndCapture => {
      let preprocessed_input = preprocessor::preprocess(input);
      let parsed_input = parse_input_regex(&preprocessed_input, &matching_expr);
      let parsed_input_2 = parsed_input.iter().map(|s| s.chars()).flatten().collect::<String>();
      let result = part1_regex_sum(&parsed_input_2);
      result
    }
    BenchImpl::Part2RegexCapture => {
      let result = part2_regex_sum(&input);
      result
    }
    BenchImpl::Part2Walkthrough => {
      let preprocessed_input = preprocessor::preprocess(input);
      let parsed_input = parse_input_walkthrough(&preprocessed_input);
      let result = calculate_sum(&parsed_input, &matching_expr);
      result
    }
  };
  let elapsed = now.elapsed();
  println!("[{fn_impl:#?}] Elapsed {elapsed:.2?}; Result: {result}");
}

pub fn run(input: &str, env: Env) {
  bench_spacer(env);
  run_benchmark(input, BenchImpl::Part1Walkthrough);
  run_benchmark(input, BenchImpl::Part1RegexParse);
  run_benchmark(input, BenchImpl::Part1RegexCapture);
  run_benchmark(input, BenchImpl::Part2Walkthrough);
  run_benchmark(input, BenchImpl::Part2RegexParse);
  run_benchmark(input, BenchImpl::Part2RegexCapture);
  run_benchmark(input, BenchImpl::Part2RegexPreprocessAndCapture);
}
