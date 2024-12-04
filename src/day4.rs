pub mod benchmarks;

const LOOKUP_XMAS: &str = "XMAS";
const ALL_DIRECTION_LOOKUP_DIFFS: [(i16, i16); 8] =
  [(-1, 0), (0, 1), (1, 0), (0, -1), (-1, 1), (1, 1), (1, -1), (-1, -1)];

fn goes_out_of_bounds(i_count: usize, j_count: usize, i_lookup: i16, j_lookup: i16) -> bool {
  i_lookup < 0
    || usize::try_from(i_lookup).ok().unwrap() >= i_count
    || j_lookup < 0
    || usize::try_from(j_lookup).ok().unwrap() >= j_count
}

fn xmas_direction_lookup(
  input: &Vec<Vec<char>>,
  i_count: usize,
  j_count: usize,
  i: &i16,
  j: &i16,
  lookup_diff: (i16, i16),
) -> bool {
  let (x_diff, y_diff) = lookup_diff;

  let max_diff = i16::try_from(LOOKUP_XMAS.len()).unwrap() - 1;
  let (max_i_lookup, max_j_lookup) = (i + (max_diff * x_diff), j + (max_diff * y_diff));

  if goes_out_of_bounds(i_count, j_count, max_i_lookup, max_j_lookup) {
    return false;
  }

  let lookup_coords: Vec<(i16, i16)> = (0..=max_diff)
    .map(|diff| {
      let (i_diff, j_diff) = (diff * x_diff, diff * y_diff);
      let (i_lookup, j_lookup) = (i + i_diff, j + j_diff);
      return (i_lookup, j_lookup);
    })
    .collect();

  LOOKUP_XMAS.chars().enumerate().fold(true, |is_match, (idx, c)| {
    let lookup_at = lookup_coords[idx];
    let (i, j) = (lookup_at.0 as usize, lookup_at.1 as usize);
    is_match && c == input[i][j]
  })
}

fn xmas_lookup(input: &Vec<Vec<char>>, i_count: usize, j_count: usize, i: &i16, j: &i16) -> usize {
  ALL_DIRECTION_LOOKUP_DIFFS.into_iter().fold(0, |count, lookup_diff| {
    match xmas_direction_lookup(input, i_count, j_count, i, j, lookup_diff) {
      true => count + 1,
      false => count,
    }
  })
}

fn count_xmas_iter(input: &Vec<Vec<char>>) -> usize {
  input.iter().enumerate().fold(0, |count_i, (idx_i, row)| {
    let (i_count, j_count) = (input.len(), row.len());
    count_i
      + row.iter().enumerate().fold(0, |count_j, (idx_j, c)| {
        count_j
          + match *c == 'X' {
            true => {
              xmas_lookup(input, i_count, j_count, &i16::try_from(idx_i).unwrap(), &i16::try_from(idx_j).unwrap())
            }
            false => 0,
          }
      })
  })
}

fn count_xmas_index(input: &Vec<Vec<char>>) -> usize {
  let mut xmas_count = 0;
  let i_count = input.len();
  for i in 0..i_count {
    let j_count = input[i].len();
    for j in 0..j_count {
      if input[i][j] == 'X' {
        xmas_count += xmas_lookup(input, i_count, j_count, &i16::try_from(i).unwrap(), &i16::try_from(j).unwrap());
      }
    }
  }
  return xmas_count;
}

fn count_xmas(input: &Vec<Vec<char>>) -> usize {
  count_xmas_index(input)
}

// =====================================================================================================================

fn cross_mas_direction_lookup(input: &Vec<Vec<char>>, i: &i16, j: &i16, lookup_diffs: &Vec<(i16, i16)>) -> String {
  lookup_diffs
    .iter()
    .map(|(diff_i, diff_j)| (i + diff_i, j + diff_j))
    .map(|(i, j)| input[i as usize][j as usize])
    .collect()
}

fn cross_mas_lookup(input: &Vec<Vec<char>>, i: &i16, j: &i16) -> bool {
  let nw_se_diffs: Vec<(i16, i16)> = [(-1, -1), (0, 0), (1, 1)].to_vec();
  let ne_sw_diffs: Vec<(i16, i16)> = [(-1, 1), (0, 0), (1, -1)].to_vec();
  [ne_sw_diffs, nw_se_diffs]
    .into_iter()
    .map(|lookup_diffs| cross_mas_direction_lookup(input, i, j, &lookup_diffs))
    .all(|lookup_result| ["MAS", "SAM"].contains(&lookup_result.as_str()))
}

fn count_cross_mas(input: &Vec<Vec<char>>) -> usize {
  let mut xmas_count = 0;
  let i_count = input.len();

  // ## skip first & last row & column
  for i in 1..(i_count - 1) {
    let j_count = input[i].len();
    for j in 1..(j_count - 1) {
      if input[i][j] == 'A' {
        let lookup_result = cross_mas_lookup(input, &i16::try_from(i).unwrap(), &i16::try_from(j).unwrap());
        if lookup_result {
          xmas_count += 1
        }
      }
    }
  }
  return xmas_count;
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
  let parsed_input = input
    .split("\n")
    .filter(|x| !x.is_empty())
    .map(|line| line.chars().collect::<Vec<char>>())
    .collect::<Vec<Vec<char>>>();
  return parsed_input;
}

// =====================================================================================================================

pub fn part1(input: &str) -> usize {
  let parsed_input = parse_input(input);
  count_xmas(&parsed_input)
}

pub fn part2(input: &str) -> usize {
  let parsed_input = parse_input(input);
  count_cross_mas(&parsed_input)
}
