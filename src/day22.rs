pub mod benchmarks;

fn parse_input(input: &str) -> Vec<usize> {
  input.lines().map(|s| s.parse::<usize>().unwrap()).collect()
}

// To mix a value into the secret number, calculate the bitwise XOR of the given value
// and the secret number. Then, the secret number becomes the result of that operation.
fn mix_value_into(mix_value: usize, secret_num: usize) -> usize {
  mix_value ^ secret_num
}

// To prune the secret number, calculate the value of the secret number modulo
// 16777216. Then, the secret number becomes the result of that operation.
fn prune(secret_num: usize) -> usize {
  secret_num % 16777216
}

// Each secret number evolves into the next secret number in the sequence via
// the following process:
// 1) Calculate the result of multiplying the secret number by 64. Then, mix
//    this result into the secret number. Finally, prune the secret number.
// 2) Calculate the result of dividing the secret number by 32. Round the
//    result down to the nearest integer. Then, mix this result into the
//    secret number. Finally, prune the secret number.
// 3) Calculate the result of multiplying the secret number by 2048. Then,
//    mix this result into the secret number. Finally, prune the secret number.
fn calculate_next_number(secret_num: usize) -> usize {
  let mix_value = secret_num * 64;
  let mixed = mix_value_into(mix_value, secret_num);
  let secret_num = prune(mixed);

  let mix_value = secret_num / 32;
  let mixed = mix_value_into(mix_value, secret_num);
  let secret_num = prune(mixed);

  let mix_value = secret_num * 2048;
  let mixed = mix_value_into(mix_value, secret_num);
  let secret_num = prune(mixed);

  secret_num
}

fn calculate_nth_secret_numbers(initial_numbers: &Vec<usize>, n: usize) -> Vec<usize> {
  initial_numbers
    .into_iter()
    .map(|num| {
      let &(mut nth_result) = num;
      for _ in 0..n {
        nth_result = calculate_next_number(nth_result);
      }
      nth_result
    })
    .collect()
}

pub fn part1(input: &str) -> usize {
  let n = 2000;
  let initial_numbers = parse_input(input);
  let nth_secret_numbers = calculate_nth_secret_numbers(&initial_numbers, n);
  let result = nth_secret_numbers.into_iter().sum();
  result
}

#[cfg(test)]
mod tests {
  use super::*;

  // (If the secret number is 100000000 and you were to prune the
  // secret number, the secret number would become 16113920.)
  #[test]
  fn test_prune() {
    let test_num = 100000000;
    let result = prune(test_num);
    assert_eq!(result, 16113920);
  }

  // (If the secret number is 42 and you were to mix 15 into the
  // secret number, the secret number would become 37.)
  #[test]
  fn test_mix() {
    let (test_num, mix_value) = (42, 15);
    let result = mix_value_into(mix_value, test_num);
    assert_eq!(result, 37);
  }

  #[test]
  fn test_calculate_next_number() {
    let mut initial = 123;
    let result = calculate_next_number(initial);
    assert_eq!(result, 15887950);

    let nth_results = [
      15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432, 5908254,
    ];
    for i in 0..nth_results.len() {
      let result = calculate_next_number(initial);
      assert_eq!(result, nth_results[i]);
      initial = result;
    }
  }
}
