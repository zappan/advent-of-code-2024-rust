use super::*;
use crate::util::io::{bench_spacer, Env};

#[derive(Debug)]
enum BenchImpl {
  Part1Std,
}

fn run_benchmark(input: &str, fn_impl: BenchImpl) {
  let now = std::time::Instant::now();
  let result = match fn_impl {
    BenchImpl::Part1Std => {
      let result = part1(input);
      result
    }
  };
  let elapsed = now.elapsed();
  println!("[{fn_impl:#?}] Elapsed {elapsed:.2?}; Result: {result}");
}

pub fn run(input: &str, env: Env) {
  bench_spacer(env);
  run_benchmark(input, BenchImpl::Part1Std);
}
