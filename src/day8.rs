use std::collections::HashMap;
use std::ops::Range;

use itertools::Itertools;

pub mod benchmarks;

#[derive(Debug, Copy, Clone, Default)]
struct FieldSize {
  x: usize,
  y: usize,
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash)]
struct Pos {
  i: usize,
  j: usize,
}

#[derive(Debug, Copy, Clone, Default)]
struct Antenna {
  pos: Pos,
  freq_marker: char,
}

fn is_within_bounds(field_size: &FieldSize, i: i64, j: i64) -> bool {
  i >= 0 && i < field_size.x as i64 && j >= 0 && j < field_size.y as i64
}

fn calculate_resonant_freqs_antinodes(
  grouped_antennas: HashMap<char, Vec<Pos>>,
  field_size: FieldSize,
  antinode_dist_multipliers: &Range<usize>,
) -> Vec<Pos> {
  grouped_antennas
    .into_iter()
    .flat_map(|(_freq_marker, locations)| {
      locations.into_iter().combinations(2).flat_map(|pair| {
        pair.into_iter().permutations(2).flat_map(|pair| {
          let mut resonant_antinodes = vec![];
          let (l1, l2) = (pair[0], pair[1]);
          let diff_i = l2.i as i64 - l1.i as i64;
          let diff_j = l2.j as i64 - l1.j as i64;

          for an_dist_multiplier in antinode_dist_multipliers.clone() {
            let an_i = l1.i as i64 + an_dist_multiplier as i64 * diff_i;
            let an_j = l1.j as i64 + an_dist_multiplier as i64 * diff_j;
            if is_within_bounds(&field_size, an_i, an_j) {
              let an = Pos {
                i: an_i as usize,
                j: an_j as usize,
              };
              resonant_antinodes.push(an);
            } else {
              break; // terminates early, as all following iterations are even more out-of-bounds
            }
          }
          resonant_antinodes
        })
      })
    })
    .unique()
    .collect::<Vec<Pos>>()
}

fn group_antennas(antennas: &Vec<Antenna>) -> HashMap<char, Vec<Pos>> {
  antennas
    .into_iter()
    .fold(HashMap::<char, Vec<Pos>>::new(), |mut map, a| {
      match map.contains_key(&a.freq_marker) {
        true => drop(map.get_mut(&a.freq_marker).unwrap().push(a.pos)),
        false => drop(map.insert(a.freq_marker, vec![a.pos])),
      };
      return map;
    })
}

fn parse_input(input: &str) -> (FieldSize, Vec<Antenna>) {
  let rows_iter = input.split("\n").filter(|x| !x.is_empty());

  let field_size = FieldSize {
    x: rows_iter.clone().count(),
    y: rows_iter.clone().nth(1).unwrap().len(),
  };

  let antennas = rows_iter
    .enumerate()
    .flat_map(|(row_idx, line)| {
      line
        .char_indices()
        .filter(|(_, c)| c.is_ascii_alphanumeric())
        .map(move |(col_idx, c)| Antenna {
          pos: { Pos { i: row_idx, j: col_idx } },
          freq_marker: c,
          ..Default::default()
        })
    })
    .collect::<Vec<Antenna>>();

  (field_size, antennas)
}

pub fn part1(input: &str) -> usize {
  let (field_size, antennas) = parse_input(input);
  let grouped_antennas = group_antennas(&antennas);
  let antinode_dist_multipliers = 2..3_usize;
  let antinodes = calculate_resonant_freqs_antinodes(grouped_antennas, field_size, &antinode_dist_multipliers);
  let result = antinodes.len();
  result
}

pub fn part2(input: &str) -> usize {
  let (field_size, antennas) = parse_input(input);
  let grouped_antennas = group_antennas(&antennas);
  let antinode_dist_multipliers = 0..std::cmp::min(field_size.x, field_size.y);
  let antinodes = calculate_resonant_freqs_antinodes(grouped_antennas, field_size, &antinode_dist_multipliers);
  let result = antinodes.len();
  result
}
