pub mod benchmarks;

use regex::Regex;
use statistical;
use std::collections::HashMap;

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
pub struct Matrix {
  pub x: i64,
  pub y: i64,
}

#[derive(Debug)]
struct Robot {
  position: Matrix,
  velocity: Matrix,
}

impl Robot {
  fn make_n_moves(&mut self, n: usize, grid_size: &Matrix) {
    let dnx = self.velocity.x * n as i64;
    let dny = self.velocity.y * n as i64;
    self.position.x = (self.position.x + dnx) % grid_size.x;
    self.position.y = (self.position.y + dny) % grid_size.y;
  }
}

fn calc_positions_after_n_moves(
  robots_movements: &Vec<String>,
  grid_size: Matrix,
  moves_count: usize,
) -> HashMap<Matrix, usize> {
  let mut robo_map: HashMap<Matrix, usize> = HashMap::new();

  let capture_regex = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
  robots_movements
    .into_iter()
    .map(|robot_input| {
      let capture = capture_regex.captures(&robot_input).unwrap();
      Robot {
        position: Matrix {
          x: capture.get(1).unwrap().as_str().parse::<i64>().unwrap(),
          y: capture.get(2).unwrap().as_str().parse::<i64>().unwrap(),
        },
        velocity: Matrix {
          x: ((capture.get(3).unwrap().as_str().parse::<i64>().unwrap() + grid_size.x) % grid_size.x),
          y: ((capture.get(4).unwrap().as_str().parse::<i64>().unwrap() + grid_size.y) % grid_size.y),
        },
      }
    })
    .map(|mut r| {
      r.make_n_moves(moves_count, &grid_size);
      r.position
    })
    .for_each(|pos| {
      if let Some(count) = robo_map.get_mut(&pos) {
        *count += 1;
      } else {
        robo_map.insert(pos, 1);
      }
    });

  robo_map
}

fn calc_safety_factor(robo_map: HashMap<Matrix, usize>, grid_size: Matrix) -> usize {
  let (mid_x, mid_y) = (grid_size.x / 2, grid_size.y / 2);
  let mut quadrants = HashMap::from([(1, 0), (2, 0), (3, 0), (4, 0)]);
  robo_map
    .into_iter()
    .filter(|(k, _)| k.x != mid_x && k.y != mid_y)
    .for_each(|(k, v)| {
      let is_q1 = k.x < mid_x && k.y < mid_y;
      let is_q2 = k.x > mid_x && k.y < mid_y;
      let is_q3 = k.x < mid_x && k.y > mid_y;
      let is_q4 = k.x > mid_x && k.y > mid_y;

      let q = if is_q1 {
        1
      } else if is_q2 {
        2
      } else if is_q3 {
        3
      } else if is_q4 {
        4
      } else {
        return;
      };
      let count = quadrants.get_mut(&q).unwrap();
      *count += v;
    });
  quadrants.values().product()
}

fn calc_variance(robo_map: HashMap<Matrix, usize>, mean_x: i64) -> f64 {
  let robots_x_axes = robo_map.into_iter().map(|(pos, _)| pos.x as f64).collect::<Vec<f64>>();
  statistical::population_variance(&robots_x_axes, Some(mean_x as f64))
}

fn parse_input(input: &str) -> Vec<String> {
  input.trim().lines().map(|s| s.to_string()).collect()
}

pub fn part1(input: &str) -> usize {
  let moves_count = 100;
  let grid_size = Matrix { x: 101, y: 103 };

  let robots_movements = parse_input(input);
  let robo_map_after_n_moves = calc_positions_after_n_moves(&robots_movements, grid_size, moves_count);
  let result = calc_safety_factor(robo_map_after_n_moves, grid_size);
  result
}

pub fn part2(input: &str) -> usize {
  let grid_size = Matrix { x: 101, y: 103 };
  let mean_x = grid_size.x / 2;

  let robots_movements = parse_input(input);

  let mut min_variance: (usize, f64) = (0, f64::MAX);
  let repeats_after = (grid_size.x * grid_size.y) as usize;
  for moves_count in 0..repeats_after as usize {
    let robo_map_after_n_moves = calc_positions_after_n_moves(&robots_movements, grid_size, moves_count);
    let variance = calc_variance(robo_map_after_n_moves, mean_x);
    if variance < min_variance.1 {
      min_variance = (moves_count, variance);
    }
  }

  let result = min_variance.0;
  result
}
