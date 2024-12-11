use crate::day11::via_iterative;
use crate::day11::via_memoized_tree_traversal;
use crate::day11::via_tree_traversal;
use crate::util::io::{bench_spacer, Env};

#[derive(Debug)]
enum BenchImpl {
  Part1Iterative,
  Part1TreeTraversal,
  Part1MemoizedTreeTraversal,
  // Part2Iterative,
  // Part2TreeTraversal,
  Part2MemoizedTreeTraversal,
}

fn run_benchmark(input: &str, fn_impl: BenchImpl) {
  let now = std::time::Instant::now();
  let result = match fn_impl {
    BenchImpl::Part1Iterative => {
      let result = via_iterative::part1(input);
      result
    }
    BenchImpl::Part1TreeTraversal => {
      let result = via_tree_traversal::part1(input);
      result
    }
    BenchImpl::Part1MemoizedTreeTraversal => {
      let result = via_memoized_tree_traversal::part1(input);
      result
    }
    // BenchImpl::Part2Iterative => {
    //   let result = via_iterative::part2(input);
    //   result
    // }
    // BenchImpl::Part2TreeTraversal => {
    //   let result = via_tree_traversal::part2(input);
    //   result
    // }
    BenchImpl::Part2MemoizedTreeTraversal => {
      let result = via_memoized_tree_traversal::part2(input);
      result
    }
  };
  let elapsed = now.elapsed();
  println!("[{fn_impl:#?}] Elapsed {elapsed:.2?}; Result: {result}");
}

pub fn run(input: &str, env: Env) {
  bench_spacer(env);
  run_benchmark(input, BenchImpl::Part1Iterative);
  run_benchmark(input, BenchImpl::Part1TreeTraversal);
  run_benchmark(input, BenchImpl::Part1MemoizedTreeTraversal);
  // ### THIS ONE TAKES AGES TO COMPLETE, SO IT'S COMMENTED-OUT
  // run_benchmark(input, BenchImpl::Part2Iterative);
  // run_benchmark(input, BenchImpl::Part2TreeTraversal);
  run_benchmark(input, BenchImpl::Part2MemoizedTreeTraversal);
}
