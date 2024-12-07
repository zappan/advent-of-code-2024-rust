use super::*;
use crate::util::io::{bench_spacer, Env};

#[derive(Debug)]
enum BenchImpl {
  Part1Std,
  Part2Std,
}

fn run_benchmark(input: &str, fn_impl: BenchImpl) {
  let now = std::time::Instant::now();
  let result = match fn_impl {
    BenchImpl::Part1Std => {
      let parsed_input = parse_input(input);
      let allowed_ops: Vec<fn(usize, usize) -> usize> = vec![add, mul];
      let result = validate_equations(parsed_input, allowed_ops);
      result
    }
    BenchImpl::Part2Std => {
      let parsed_input = parse_input(input);
      let allowed_ops: Vec<fn(usize, usize) -> usize> = vec![add, mul, concat];
      let result = validate_equations(parsed_input, allowed_ops);
      result
    }
  };
  let elapsed = now.elapsed();
  println!("[{fn_impl:#?}] Elapsed {elapsed:.2?}; Result: {result}");
}

pub fn run(input: &str, env: Env) {
  bench_spacer(env);
  run_benchmark(input, BenchImpl::Part1Std);
  run_benchmark(input, BenchImpl::Part2Std);
}
