pub mod benchmarks;

use regex::Regex;

#[derive(Debug)]
struct EquationsParams {
  ax: i64,
  ay: i64,
  px: i64,
  bx: i64,
  by: i64,
  py: i64,
}

impl EquationsParams {
  fn p_a(&self) -> i64 {
    self.py * self.ay - self.px * self.by
  }
  fn p_b(&self) -> i64 {
    self.px * self.bx - self.py * self.ax
  }
  fn q(&self) -> i64 {
    self.bx * self.ay - self.ax * self.by
  }
  fn a(&self) -> i64 {
    self.p_a() / self.q()
  }
  fn b(&self) -> i64 {
    self.p_b() / self.q()
  }
  fn has_winning_combination(&self) -> bool {
    let (p_a, p_b, q) = (self.p_a(), self.p_b(), self.q());
    p_a % q == 0 && p_b % q == 0
  }
  fn winning_combination(&self) -> (i64, i64) {
    (self.a(), self.b())
  }
  fn calculate_winning_play_cost(&self) -> usize {
    let (a, b) = self.winning_combination();
    (a * 3 + b * 1).try_into().unwrap()
  }
}

fn calc_winning_plays_cost(
  claw_machines_definitions: Vec<String>,
  winning_coords_add: i64,
  capture_regex: &Regex,
) -> usize {
  claw_machines_definitions
    .into_iter()
    .map(|claw_machine_input| {
      let capture = capture_regex.captures(&claw_machine_input).unwrap();
      EquationsParams {
        ax: capture.get(1).unwrap().as_str().parse::<i64>().unwrap(),
        bx: capture.get(2).unwrap().as_str().parse::<i64>().unwrap(),
        ay: capture.get(3).unwrap().as_str().parse::<i64>().unwrap(),
        by: capture.get(4).unwrap().as_str().parse::<i64>().unwrap(),
        px: capture.get(5).unwrap().as_str().parse::<i64>().unwrap() + winning_coords_add,
        py: capture.get(6).unwrap().as_str().parse::<i64>().unwrap() + winning_coords_add,
      }
    })
    .filter(|claw_machine| claw_machine.has_winning_combination())
    .map(|cm| cm.calculate_winning_play_cost())
    .sum()
}

fn parse_input(input: &str) -> Vec<String> {
  input.trim().split("\n\n").map(|s| s.to_string()).collect()
}

pub fn part1(input: &str) -> usize {
  let winning_coords_add: i64 = 0;
  let matching_expr =
    Regex::new(r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)").unwrap();

  let claw_machines_definitions = parse_input(input);
  calc_winning_plays_cost(claw_machines_definitions, winning_coords_add, &matching_expr)
}

pub fn part2(input: &str) -> usize {
  let winning_coords_add: i64 = 10000000000000;
  let matching_expr =
    Regex::new(r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)").unwrap();

  let claw_machines_definitions = parse_input(input);
  calc_winning_plays_cost(claw_machines_definitions, winning_coords_add, &matching_expr)
}
