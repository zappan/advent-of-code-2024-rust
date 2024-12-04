use regex::Regex;

pub mod benchmarks;
pub mod parser;
pub mod preprocessor;

fn parse_input_regex(input: &str, matching_expr: &Regex) -> Vec<String> {
  matching_expr.find_iter(input).map(|m| m.as_str().to_string()).collect()
}

fn parse_input_walkthrough(input: &str) -> Vec<String> {
  let mut parser = parser::Parser::new();
  input.chars().for_each(|c| {
    parser.consume(c);
  });

  parser.get_parsed_input().to_vec()
}

fn multiply(mul_expr: &str, capture_regex: &Regex) -> usize {
  let capture = capture_regex.captures(mul_expr).unwrap();
  let a = capture.get(1).unwrap().as_str().parse::<usize>().unwrap();
  let b = capture.get(2).unwrap().as_str().parse::<usize>().unwrap();
  a * b
}

fn calculate_sum(parsed_input: &Vec<String>, capture_regex: &Regex) -> usize {
  parsed_input
    .iter()
    .fold(0, |acc, mul_expr| acc + multiply(mul_expr, &capture_regex) as usize)
}

pub fn part1(input: &str) -> usize {
  let matching_expr = Regex::new(r"mul\((\d+)\,(\d+)\)").unwrap();
  let parsed_input: Vec<String> = parse_input_regex(input, &matching_expr);
  calculate_sum(&parsed_input, &matching_expr)
}

pub fn part2(input: &str) -> usize {
  let matching_expr = Regex::new(r"mul\((\d+)\,(\d+)\)").unwrap();
  let preprocessed_input = preprocessor::preprocess(input);
  let parsed_input: Vec<String> = parse_input_regex(&preprocessed_input, &matching_expr);
  calculate_sum(&parsed_input, &matching_expr)
}
