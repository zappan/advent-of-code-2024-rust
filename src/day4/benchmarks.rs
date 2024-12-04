use super::*;
use crate::util::io::{bench_spacer, Env};

#[derive(Debug)]
enum BenchImpl {
  Part1Index,
  Part1Iter,
  Part2Std,
}

fn run_benchmark(input: &str, fn_impl: BenchImpl) {
  let now = std::time::Instant::now();
  let result = match fn_impl {
    BenchImpl::Part1Index => {
      let parsed_input = parse_input(input);
      let result = count_xmas_index(&parsed_input);
      result
    }
    BenchImpl::Part1Iter => {
      let parsed_input = parse_input(input);
      let result = count_xmas_iter(&parsed_input);
      result
    }
    BenchImpl::Part2Std => {
      let parsed_input = parse_input(input);
      let result = count_cross_mas(&parsed_input);
      result
    }
  };
  let elapsed = now.elapsed();
  println!("[{fn_impl:#?}] Elapsed {elapsed:.2?}; Result: {result}");
}

pub fn run(input: &str, env: Env) {
  bench_spacer(env);
  run_benchmark(input, BenchImpl::Part1Index);
  run_benchmark(input, BenchImpl::Part1Iter);
  run_benchmark(input, BenchImpl::Part2Std);
}
