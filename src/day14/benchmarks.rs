use super::*;
use crate::util::io::{bench_spacer, Env};

#[derive(Debug)]
enum BenchImpl {
  Part1Std,
  Part2Std,
}

fn run_benchmark(input: &str, grid_size: Matrix, fn_impl: BenchImpl) {
  let now = std::time::Instant::now();
  let result = match fn_impl {
    BenchImpl::Part1Std => {
      let moves_count = 100;
      let robots_movements = parse_input(input);
      let robo_map = calc_positions_after_n_moves(&robots_movements, grid_size, moves_count);
      let result = calc_safety_factor(robo_map, grid_size);
      result
    }
    BenchImpl::Part2Std => {
      let mean_x = grid_size.x / 2;
      let positions_repeat_after = (grid_size.x * grid_size.y) as usize;
      let mut min_variance: (usize, f64) = (0, f64::MAX);

      let robots_movements = parse_input(input);

      for moves_count in 0..positions_repeat_after as usize {
        let robo_map_after_n_moves = calc_positions_after_n_moves(&robots_movements, grid_size, moves_count);
        let variance = calc_variance(robo_map_after_n_moves, mean_x);
        if variance < min_variance.1 {
          min_variance = (moves_count, variance);
        }
      }

      let result = min_variance.0;
      result
    }
  };
  let elapsed = now.elapsed();
  println!("[{fn_impl:#?}] Elapsed {elapsed:.2?}; Result: {result}");
}

pub fn run(input: &str, grid_size: Matrix, env: Env) {
  bench_spacer(env);
  run_benchmark(input, grid_size, BenchImpl::Part1Std);
  run_benchmark(input, grid_size, BenchImpl::Part2Std);
}
