use regex::Regex;

pub mod benchmarks;
pub mod parser;
pub mod preprocessor;

fn parse_input_regex(input: &str) -> Vec<String> {
  let matching_expr = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
  matching_expr.find_iter(input).map(|m| m.as_str().to_string()).collect()
}

fn parse_input_walkthrough(input: &str) -> Vec<String> {
  let mut parser = parser::Parser::new();
  input.chars().for_each(|c| {
    parser.consume(c);
  });

  parser.get_parsed_input().to_vec()
}

fn parse_input(input: &str) -> Vec<String> {
  parse_input_regex(input)
}

fn multiply(mul_expr: &str) -> usize {
  let re = Regex::new(r"^mul\((\d+)\,(\d+)\)$").unwrap();
  let result = re.captures(mul_expr).unwrap();
  let a = result.get(1).unwrap().as_str().parse::<usize>().unwrap();
  let b = result.get(2).unwrap().as_str().parse::<usize>().unwrap();
  a * b
}

fn calculate_sum(parsed_input: &Vec<String>) -> usize {
  parsed_input
    .iter()
    .fold(0, |acc, mul_expr| acc + multiply(mul_expr) as usize)
}

pub fn part1(input: &str) -> usize {
  let parsed_input: Vec<String> = parse_input(input);
  calculate_sum(&parsed_input)
}

pub fn part2(input: &str) -> usize {
  let preprocessed_input = preprocessor::preprocess(input);
  let parsed_input: Vec<String> = parse_input(&preprocessed_input);
  calculate_sum(&parsed_input)
}
