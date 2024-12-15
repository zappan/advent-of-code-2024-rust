use std::collections::HashMap;

pub mod benchmarks;

#[derive(Debug, Eq, Hash, PartialEq)]
enum Direction {
  N,
  E,
  S,
  W,
}

fn get_move_directions() -> HashMap<Direction, Diff> {
  HashMap::from([
    (Direction::N, Diff { x: -1, y: 0 }),
    (Direction::E, Diff { x: 0, y: 1 }),
    (Direction::S, Diff { x: 1, y: 0 }),
    (Direction::W, Diff { x: 0, y: -1 }),
  ])
}

#[derive(Debug)]
struct Diff {
  x: i64,
  y: i64,
}

#[derive(Debug, PartialEq)]
struct Coords {
  x: usize,
  y: usize,
}

struct Map {
  map_size: Coords,
  map_fields: Vec<char>,
}

fn parse_input(input: &str) -> (Map, Vec<char>) {
  let (map_input, robot_moves_input) = input.split_once("\n\n").unwrap();
  let map_input_rows_iter = map_input.lines();
  let robot_moves = robot_moves_input.replace("\n", "").chars().collect::<Vec<char>>();

  let map_size = Coords {
    x: map_input_rows_iter.clone().nth(1).unwrap().len(),
    y: map_input_rows_iter.clone().count(),
  };

  let map_fields = map_input_rows_iter
    .filter(|l| !l.is_empty())
    .flat_map(|l| l.chars())
    .collect::<Vec<char>>();

  (Map { map_size, map_fields }, robot_moves)
}

fn get_coords_from_idx(map_size: &Coords, idx: usize) -> Coords {
  let x = idx / map_size.y;
  let y = idx % map_size.y;
  Coords { x, y }
}

fn find_next_field_idx(map_size: &Coords, robot_pos_idx: &usize, diff: &Diff) -> usize {
  let next_field_idx_diff = diff.x * map_size.x as i64 + diff.y;
  let next_field_idx = *robot_pos_idx as i64 + next_field_idx_diff;
  next_field_idx as usize
}

fn find_free_spot(map: &Map, robot_pos_idx: &usize, diff: &Diff) -> Option<usize> {
  let map_fields = &map.map_fields;
  let mut gap = 1;

  loop {
    let (gap_diff_x, gap_diff_y) = (gap * diff.x, gap * diff.y);
    let map_diff = gap_diff_x * map.map_size.x as i64 + gap_diff_y;
    let free_space_idx = (*robot_pos_idx as i64 + map_diff) as usize;
    match map_fields[free_space_idx] {
      '.' => return Some(free_space_idx),
      '#' => return None,
      _ => gap += 1,
    };
  }
}

fn make_a_push(map: &mut Map, free_space_idx: usize, push_idx: usize, robot_pos_idx: usize) {
  let map_fields = &mut map.map_fields;
  map_fields.swap(free_space_idx, push_idx);
  map_fields.swap(push_idx, robot_pos_idx);
}

fn make_a_move(map: &mut Map, diff: &Diff) {
  let map_fields = &map.map_fields;
  let robot_pos_idx = map_fields.iter().position(|&c| c == '@').unwrap();
  if let Some(free_space_idx) = find_free_spot(map, &robot_pos_idx, diff) {
    let next_field_idx = find_next_field_idx(&map.map_size, &robot_pos_idx, diff);
    make_a_push(map, free_space_idx, next_field_idx, robot_pos_idx);
  }
}

fn simulate_robot_movement(map: &mut Map, robot_moves: &Vec<char>) {
  let move_directions = get_move_directions();
  let mut robot_moves_iter = robot_moves.into_iter();
  while let Some(&mv) = robot_moves_iter.next() {
    match mv {
      '^' => make_a_move(map, &move_directions[&Direction::N]),
      '>' => make_a_move(map, &move_directions[&Direction::E]),
      'v' => make_a_move(map, &move_directions[&Direction::S]),
      '<' => make_a_move(map, &move_directions[&Direction::W]),
      _ => todo!(),
    }
  }
}

fn sum_gps_coordinates(map: &Map) -> usize {
  map
    .map_fields
    .iter()
    .enumerate()
    .filter(|(_, field)| **field == 'O')
    .map(|(idx, _)| {
      let coords = get_coords_from_idx(&map.map_size, idx);
      coords.x * 100 + coords.y
    })
    .sum()
}

pub fn part1(input: &str) -> usize {
  let (mut map, robot_moves) = parse_input(input);
  simulate_robot_movement(&mut map, &robot_moves);
  let result = sum_gps_coordinates(&map);
  result
}

pub fn part2(input: &str) -> usize {
  input.len()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_coords_from_idx() {
    let map_size = Coords { x: 3, y: 5 };

    let idx = 1;
    let result = get_coords_from_idx(&map_size, idx);
    assert_eq!(result, Coords { x: 0, y: 1 });

    let idx = 7;
    let result = get_coords_from_idx(&map_size, idx);
    assert_eq!(result, Coords { x: 1, y: 2 });

    let idx = 9;
    let result = get_coords_from_idx(&map_size, idx);
    assert_eq!(result, Coords { x: 1, y: 4 });
  }
}
