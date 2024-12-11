use super::*;
use crate::util::io::{bench_spacer, Env};

#[derive(Debug)]
enum BenchImpl {
  Part1Vec,
  Part1HashMap,
  Part2Vec,
  Part2HashMap,
}

fn run_benchmark(input: &str, fn_impl: BenchImpl) {
  let now = std::time::Instant::now();
  let result = match fn_impl {
    BenchImpl::Part1Vec => {
      let result = via_vec::part1(input);
      result
    }
    BenchImpl::Part1HashMap => {
      let result = via_hashmap::part1(input);
      result
    }
    BenchImpl::Part2Vec => {
      let result = via_vec::part2(input);
      result
    }
    BenchImpl::Part2HashMap => {
      let result = via_hashmap::part2(input);
      result
    }
  };
  let elapsed = now.elapsed();
  println!("[{fn_impl:#?}] Elapsed {elapsed:.2?}; Result: {result}");
}

pub fn run(input: &str, env: Env) {
  bench_spacer(env);
  run_benchmark(input, BenchImpl::Part1Vec);
  run_benchmark(input, BenchImpl::Part1HashMap);
  run_benchmark(input, BenchImpl::Part2Vec);
  run_benchmark(input, BenchImpl::Part2HashMap);
}
