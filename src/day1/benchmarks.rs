use super::*;

#[derive(Debug)]
enum BenchImpl {
  Part1Std,
  Part2Std,
  Part2Fast,
}

fn run_benchmark(input: &str, fn_impl: BenchImpl) {
  let (list1, list2) = parse_input(input);

  let now = std::time::Instant::now();
  let result = match fn_impl {
    BenchImpl::Part1Std => calc_total_distance(list1, list2),
    BenchImpl::Part2Std => calc_similarity_score(&list1, &list2),
    BenchImpl::Part2Fast => calc_similarity_score_fast(&list1, &list2),
  };
  let elapsed = now.elapsed();
  println!("[{fn_impl:#?}] Elapsed {elapsed:.2?}; Result: {result}");
}

pub fn run(input: &str, bench_spacer: fn()) {
  bench_spacer();
  run_benchmark(input, BenchImpl::Part1Std);
  run_benchmark(input, BenchImpl::Part2Std);
  run_benchmark(input, BenchImpl::Part2Fast);
}
