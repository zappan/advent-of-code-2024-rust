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
      let (rules, updates) = parse_input(input);
      let correct_updates = get_correct_updates(&rules, &updates);
      let result: usize = correct_updates.into_iter().map(|u| get_middle_element(&u)).sum();
      result
    }
    BenchImpl::Part2Std => {
      let (rules, updates) = parse_input(input);
      let incorrect_updates = get_incorrect_updates(&rules, &updates);
      let result: usize = incorrect_updates
        .into_iter()
        .map(|u| correct_update_order(&rules, &u))
        .map(|u| get_middle_element(&u))
        .sum();
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
