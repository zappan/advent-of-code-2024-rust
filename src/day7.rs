use itertools::Itertools;

pub mod benchmarks;

#[derive(Debug)]
struct Equation {
  expected_result: usize,
  operands: Vec<usize>,
}

fn add(a: usize, b: usize) -> usize {
  a + b
}
fn mul(a: usize, b: usize) -> usize {
  a * b
}
fn concat(a: usize, b: usize) -> usize {
  // (a.to_string() + &b.to_string()).parse::<usize>().unwrap()
  // format!("{}{}", a, b).parse::<usize>().unwrap()
  a * 10_usize.pow(b.ilog10() + 1) + b
}

fn validate_equation(eq: &Equation, allowed_ops: &Vec<fn(usize, usize) -> usize>) -> bool {
  let ops_count = eq.operands.len() - 1;
  let ops_to_test: Vec<_> = (0..ops_count).map(|_| allowed_ops).multi_cartesian_product().collect();

  ops_to_test.into_iter().any(|ops_iter| {
    let (_, res) = eq
      .operands
      .clone()
      .into_iter()
      .enumerate()
      .reduce(|(_, a), (j, b)| {
        let op_idx = j - 1;
        let op = ops_iter[op_idx];
        (0, op(a.into(), b.into()))
      })
      .unwrap();

    return eq.expected_result == res;
  })
}

fn validate_equations(equations: Vec<Equation>, allowed_ops: Vec<fn(usize, usize) -> usize>) -> usize {
  equations
    .into_iter()
    .filter(|eq| validate_equation(eq, &allowed_ops))
    .map(|eq| eq.expected_result)
    .sum::<usize>()
}

fn parse_input(input: &str) -> Vec<Equation> {
  input
    .split("\n")
    .filter(|x| !x.is_empty())
    .map(|line| {
      let (result_input, operands_input) = line.split_once(":").unwrap();
      let expected_result = result_input.parse::<usize>().expect("Value must represent a number");
      let operands = operands_input
        .trim()
        .split_whitespace()
        .map(|op| op.parse::<usize>().expect("Values must represent a number"))
        .collect::<Vec<usize>>();

      return Equation {
        expected_result,
        operands,
      };
    })
    .collect::<Vec<Equation>>()
}

pub fn part1(input: &str) -> usize {
  let parsed_input = parse_input(input);
  let allowed_ops: Vec<fn(usize, usize) -> usize> = vec![add, mul];
  let result = validate_equations(parsed_input, allowed_ops);
  result
}

pub fn part2(input: &str) -> usize {
  let parsed_input = parse_input(input);
  let allowed_ops: Vec<fn(usize, usize) -> usize> = vec![add, mul, concat];
  let result = validate_equations(parsed_input, allowed_ops);
  result
}
