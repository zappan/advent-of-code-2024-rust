use itertools::Itertools;
use std::collections::{HashMap, HashSet};

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
fn calculate_next_secret(secret_num: usize) -> usize {
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

fn calculate_nth_secret(num: usize, n: usize) -> usize {
  match n {
    0 => num,
    _ => calculate_nth_secret(calculate_next_secret(num), n - 1),
  }
}

fn calculate_nth_secrets(initial_numbers: &Vec<usize>, n: usize) -> Vec<usize> {
  initial_numbers
    .into_iter()
    .map(|&num| calculate_nth_secret(num, n))
    .collect()
}

fn calculate_sequence(num: usize, seq_len: usize) -> (i8, i8, i8, i8) {
  let mut prev_num = num;
  (0..seq_len)
    .map(|_| {
      let next_num = calculate_next_secret(prev_num);
      let (prev_last_digit, next_last_digit) = (prev_num % 10, next_num % 10);
      let diff = next_last_digit as i8 - prev_last_digit as i8;
      prev_num = next_num;
      diff
    })
    .collect_tuple()
    .unwrap()
}

fn collect_sequences_results(
  initial_numbers: &Vec<usize>,
  secrets_per_day: usize,
) -> (Vec<HashMap<(i8, i8, i8, i8), usize>>, HashSet<(i8, i8, i8, i8)>) {
  let mut uniq_seqs: HashSet<(i8, i8, i8, i8)> = HashSet::new();

  let buyers_seqs: Vec<HashMap<(i8, i8, i8, i8), usize>> = initial_numbers
    .into_iter()
    .map(|&initial_num| {
      let mut sequences: HashMap<(i8, i8, i8, i8), usize> = HashMap::new();
      let mut num: usize = initial_num;

      // ## initial + n genereated secret numbers
      let iter_max = 1 + secrets_per_day - SEQ_LEN;
      for _ in 0..iter_max {
        let seq = calculate_sequence(num, SEQ_LEN);
        if !sequences.contains_key(&seq) {
          let seq_result = calculate_nth_secret(num, SEQ_LEN);
          let price = seq_result % 10;
          sequences.insert(seq, price);
        }

        // mini-optimization: if last diff is less than 0, no need to check that
        // seq as its price is definitely less than in the previous iteration
        if seq.3 >= 0 {
          uniq_seqs.insert(seq);
        }

        num = calculate_next_secret(num);
      }
      sequences
    })
    .collect();

  (buyers_seqs, uniq_seqs)
}

const SECRETS_PER_DAY: usize = 2000;
const SEQ_LEN: usize = 4;

pub fn part1(input: &str) -> usize {
  let initial_numbers = parse_input(input);
  let nth_secret_numbers = calculate_nth_secrets(&initial_numbers, SECRETS_PER_DAY);
  let result = nth_secret_numbers.into_iter().sum();
  result
}

pub fn part2(input: &str) -> usize {
  let initial_numbers = parse_input(input);
  let (buyers_seqs, uniq_seqs) = collect_sequences_results(&initial_numbers, SECRETS_PER_DAY);

  // foreach seq in uniq_seq
  //   find first such seq in each buyers sequences
  //   and sum up its prices
  // max price is the result
  let max_sum: usize = uniq_seqs
    .into_iter()
    .map(|seq| {
      buyers_seqs
        .iter()
        .map(|buyer_seqs| match buyer_seqs.get(&seq) {
          Some(&price) => price,
          None => 0,
        })
        .sum::<usize>()
    })
    .max()
    .unwrap();

  let result = max_sum;
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
  fn test_calculate_next_secret() {
    let mut initial = 123;
    let result = calculate_next_secret(initial);
    assert_eq!(result, 15887950);

    let nth_results = [
      15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432, 5908254,
    ];
    for i in 0..nth_results.len() {
      let result = calculate_next_secret(initial);
      assert_eq!(result, nth_results[i]);
      initial = result;
    }
  }

  #[test]
  fn test_calculate_sequence() {
    let mut num = 123;
    let result = calculate_sequence(num, SEQ_LEN);
    assert_eq!(result, (-3, 6, -1, -1));

    num = calculate_next_secret(num);
    let result = calculate_sequence(num, SEQ_LEN);
    assert_eq!(result, (6, -1, -1, 0));

    num = calculate_next_secret(num);
    let result = calculate_sequence(num, SEQ_LEN);
    assert_eq!(result, (-1, -1, 0, 2));

    num = calculate_next_secret(num);
    let result = calculate_sequence(num, SEQ_LEN);
    assert_eq!(result, (-1, 0, 2, -2));
  }
}
