use std::collections::HashMap;

pub mod benchmarks;

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
enum Dir {
  N,
  E,
  S,
  W,
}

struct MoveDirection {
  dir: Dir,
  marker: char,
  diff: (i16, i16),
  on_obstacle: Dir,
}

const MOVE_DIRECTIONS: [MoveDirection; 4] = [
  MoveDirection {
    dir: Dir::N,
    marker: '^',
    diff: (-1, 0),
    on_obstacle: Dir::E,
  },
  MoveDirection {
    dir: Dir::E,
    marker: '>',
    diff: (0, 1),
    on_obstacle: Dir::S,
  },
  MoveDirection {
    dir: Dir::S,
    marker: 'v',
    diff: (1, 0),
    on_obstacle: Dir::W,
  },
  MoveDirection {
    dir: Dir::W,
    marker: '<',
    diff: (0, -1),
    on_obstacle: Dir::N,
  },
];

#[derive(Debug, Copy, Clone, Default)]
struct Pos {
  i: usize,
  j: usize,
}

#[derive(Debug, Copy, Clone)]
struct Guard {
  pos: Pos,
  dir: Dir,
  loop_counter: u8,
}

impl Default for Guard {
  fn default() -> Self {
    Self {
      pos: Pos::default(),
      dir: Dir::N,
      loop_counter: 0,
    }
  }
}

fn get_walked_fields_without_initial(matrix: &Vec<Vec<char>>, initial: &Pos) -> Vec<Pos> {
  let mut walked_fields: Vec<Pos> = vec![];
  for i in 0..matrix.len() {
    for j in 0..matrix[i].len() {
      let is_initial_pos = i == initial.i && j == initial.j;
      if matrix[i][j] == 'X' && !is_initial_pos {
        walked_fields.push(Pos { i, j });
      }
    }
  }
  walked_fields
}

fn goes_out_of_bounds(i_count: usize, j_count: usize, i_lookup: i16, j_lookup: i16) -> bool {
  i_lookup < 0
    || usize::try_from(i_lookup).ok().unwrap() >= i_count
    || j_lookup < 0
    || usize::try_from(j_lookup).ok().unwrap() >= j_count
}

fn simulate_guard_movement(matrix: &mut Vec<Vec<char>>, guard: &mut Guard) {
  let (matrix_size_i, matrix_size_j) = (matrix.len(), matrix[0].len());

  let move_direction_diffs: HashMap<Dir, (i16, i16)> = MOVE_DIRECTIONS.map(|md| (md.dir, md.diff)).into();
  let move_direction_on_obstacle: HashMap<Dir, Dir> = MOVE_DIRECTIONS.map(|md| (md.dir, md.on_obstacle)).into();

  loop {
    let (i, j) = (guard.pos.i, guard.pos.j);
    let look_ahead_diff = move_direction_diffs.get(&guard.dir).unwrap();
    let look_ahead_i = &i16::try_from(i).unwrap() + look_ahead_diff.0;
    let look_ahead_j = &i16::try_from(j).unwrap() + look_ahead_diff.1;

    if goes_out_of_bounds(matrix_size_i, matrix_size_j, look_ahead_i, look_ahead_j) {
      // ## mark current field as 'walked' and leave the loop
      matrix[i][j] = 'X';
      break;
    }

    if matrix[look_ahead_i as usize][look_ahead_j as usize] == '#' {
      // ## if walking on walked path and already turned 4 times, it's a detected loop
      if guard.loop_counter == 4 {
        break;
      }
      // ## guard turns
      guard.loop_counter += 1;
      guard.dir = *move_direction_on_obstacle.get(&guard.dir).unwrap();
    } else {
      // mark current field as 'walked' and move into next position
      if matrix[i][j] == '.' {
        guard.loop_counter = 0;
      }
      matrix[i][j] = 'X';
      guard.pos = Pos {
        i: look_ahead_i as usize,
        j: look_ahead_j as usize,
      };
    }
  }
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Guard) {
  let direction_markers = MOVE_DIRECTIONS.map(|md| md.marker);
  let mut guard_initial_position = Default::default();

  let matrix = input
    .split("\n")
    .filter(|x| !x.is_empty())
    .enumerate()
    .map(|(row_idx, line)| {
      if let Some(col_idx) = line.find(|c| direction_markers.contains(&c)) {
        let guard_marker = line.chars().nth(col_idx).unwrap();
        guard_initial_position = Guard {
          pos: { Pos { i: row_idx, j: col_idx } },
          dir: MOVE_DIRECTIONS.iter().find(|md| md.marker == guard_marker).unwrap().dir,
          ..Default::default()
        };
      }
      line.chars().collect::<Vec<char>>()
    })
    .collect::<Vec<Vec<char>>>();

  (matrix, guard_initial_position)
}

fn get_guard_distinct_positions_count(matrix: &Vec<Vec<char>>) -> usize {
  matrix.into_iter().flatten().filter(|&pos| *pos == 'X').count()
}

fn calc_obstruction_count_parse(input: &str, walked_matrix: &Vec<Vec<char>>, guard_initial_position: &Pos) -> usize {
  get_walked_fields_without_initial(&walked_matrix, guard_initial_position)
    .into_iter()
    .fold(0, |obstruction_count, wf| {
      // ## re-parse inputs to reset the matrix after each run, otherwise the obstacles and walked path would stay...
      let (mut matrix, mut guard) = parse_input(input);
      matrix[wf.i][wf.j] = '#'; // ## add obstacle
      simulate_guard_movement(&mut matrix, &mut guard); // ## trigger patrol
      match guard.loop_counter {
        4 => obstruction_count + 1,
        _ => obstruction_count,
      }
    })
}

fn calc_obstruction_count_clone(walked_matrix: &Vec<Vec<char>>, matrix: &Vec<Vec<char>>, guard: &Guard) -> usize {
  get_walked_fields_without_initial(&walked_matrix, &guard.pos)
    .into_iter()
    .fold(0, |obstruction_count, wf| {
      // ## clone inputs need to reset the matrix after each run, otherwise the obstacles and walked path would stay...
      let (mut matrix, mut guard) = (matrix.clone(), guard.clone());
      matrix[wf.i][wf.j] = '#'; // ## add obstacle
      simulate_guard_movement(&mut matrix, &mut guard); // ## trigger patrol
      match guard.loop_counter {
        4 => obstruction_count + 1,
        _ => obstruction_count,
      }
    })
}

pub fn part1(input: &str) -> usize {
  let (mut matrix, mut guard_initial_position) = parse_input(input);
  simulate_guard_movement(&mut matrix, &mut guard_initial_position);
  let result = get_guard_distinct_positions_count(&matrix);
  result
}

pub fn part2(input: &str) -> usize {
  let (initial_matrix, initial_guard) = parse_input(input);
  let (mut matrix, mut guard) = (initial_matrix.clone(), initial_guard.clone());
  simulate_guard_movement(&mut matrix, &mut guard);
  let result = calc_obstruction_count_clone(&matrix, &initial_matrix, &initial_guard);
  result
}
