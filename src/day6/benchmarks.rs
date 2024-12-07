use super::*;
use crate::util::io::{bench_spacer, Env};

#[derive(Debug)]
enum BenchImpl {
  Part1Std,
  Part2InputParsing,
  Part2InputCloning,
}

fn run_benchmark(input: &str, fn_impl: BenchImpl) {
  let now = std::time::Instant::now();
  let result = match fn_impl {
    BenchImpl::Part1Std => {
      let (mut matrix, mut guard_initial_position) = parse_input(input);
      simulate_guard_movement(&mut matrix, &mut guard_initial_position);
      let result = get_guard_distinct_positions_count(&matrix);
      result
    }
    BenchImpl::Part2InputParsing => {
      let (mut matrix, mut guard_initial_position) = parse_input(input);
      simulate_guard_movement(&mut matrix, &mut guard_initial_position);
      let result = calc_obstruction_count_parse(&input, &matrix, &guard_initial_position.pos);
      result
    }
    BenchImpl::Part2InputCloning => {
      let (initial_matrix, initial_guard) = parse_input(input);
      let (mut matrix, mut guard) = (initial_matrix.clone(), initial_guard.clone());
      simulate_guard_movement(&mut matrix, &mut guard);
      let result = calc_obstruction_count_clone(&matrix, &initial_matrix, &initial_guard);
      result
    }
  };
  let elapsed = now.elapsed();
  println!("[{fn_impl:#?}] Elapsed {elapsed:.2?}; Result: {result}");
}

pub fn run(input: &str, env: Env) {
  bench_spacer(env);
  run_benchmark(input, BenchImpl::Part1Std);
  run_benchmark(input, BenchImpl::Part2InputParsing);
  run_benchmark(input, BenchImpl::Part2InputCloning);
}
