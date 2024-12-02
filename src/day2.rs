use std::fs;

pub mod benchmarks;

const INPUT_FILE: &str = "input/day02.txt";
const TEST_INPUT_FILE: &str = "input/test/day02.txt";

pub fn read_input_file() -> String {
  let file_contents = fs::read_to_string(INPUT_FILE).unwrap();
  return file_contents;
}

pub fn _read_test_input_file() -> String {
  let file_contents = fs::read_to_string(TEST_INPUT_FILE).unwrap();
  return file_contents;
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
  let parsed_input = input
    .split("\n")
    .filter(|x| !x.is_empty())
    .map(|line| {
      line
        .split_whitespace()
        .map(|x| x.parse::<u8>().unwrap())
        .collect::<Vec<u8>>()
    })
    .collect::<Vec<Vec<u8>>>();
  return parsed_input;
}

fn validate_line_item(line_item: &u8, prev_line_item: &u8, expected_direction: std::cmp::Ordering) -> bool {
  let level_direction = line_item.cmp(&prev_line_item);
  let diff = match level_direction {
    std::cmp::Ordering::Greater => line_item - prev_line_item,
    std::cmp::Ordering::Less => prev_line_item - line_item,
    std::cmp::Ordering::Equal => 0,
  };

  let is_safe_levels_direction = level_direction == expected_direction;
  let is_safe_level_diff = 1 <= diff && diff <= 3;
  return is_safe_levels_direction && is_safe_level_diff;
}

fn validate_report_line(report_line: &mut Vec<u8>) -> bool {
  let expected_direction = report_line[1].cmp(&report_line[0]);
  (1..report_line.len()).fold(true, |is_line_safe, i| {
    let (line_item, prev_line_item) = (report_line[i], report_line[i - 1]);
    return is_line_safe && validate_line_item(&line_item, &prev_line_item, expected_direction);
  })
}

fn apply_line_validation_dampener(report_line: &mut Vec<u8>) -> bool {
  (0..report_line.len())
    .map(|i| {
      let mut report_line_clone = report_line.clone();
      report_line_clone.remove(i);
      validate_report_line(&mut report_line_clone)
    })
    .any(|x| x)
}

fn validate_report_line_with_dampener(report_line: &mut Vec<u8>) -> bool {
  match validate_report_line(report_line) {
    true => true,
    false => apply_line_validation_dampener(report_line),
  }
}

fn count_safe_reports(reports_data: &mut Vec<Vec<u8>>, line_validator: impl Fn(&mut Vec<u8>) -> bool) -> u16 {
  reports_data.iter_mut().map(line_validator).filter(|x| *x).count() as u16
}

pub fn part1(input: &str) -> u16 {
  let mut reports_data: Vec<Vec<u8>> = parse_input(input);
  count_safe_reports(&mut reports_data, validate_report_line)
}

pub fn part2(input: &str) -> u16 {
  let mut reports_data: Vec<Vec<u8>> = parse_input(input);
  count_safe_reports(&mut reports_data, validate_report_line_with_dampener)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_validates_report_lines() {
    // # ascending: safe line items
    assert_eq!(validate_report_line(&mut vec![1, 2, 3, 4]), true);
    assert_eq!(validate_report_line(&mut vec![1, 4, 7, 10]), true);
    // # descending: safe line items
    assert_eq!(validate_report_line(&mut vec![4, 3, 2, 1]), true);
    assert_eq!(validate_report_line(&mut vec![10, 7, 4, 1]), true);

    // # ascending: unsafe first line item diff
    assert_eq!(validate_report_line(&mut vec![1, 12, 13, 14]), false);
    // # ascending: unsafe first line item sets descending direction
    assert_eq!(validate_report_line(&mut vec![21, 12, 13, 14]), false);
    // # descending: unsafe first line item diff
    assert_eq!(validate_report_line(&mut vec![31, 14, 13, 12]), false);
    // # descending: unsafe first line item sets ascending direction
    assert_eq!(validate_report_line(&mut vec![1, 14, 11, 9]), false);

    // # ascending: unsafe 2nd line item diff
    assert_eq!(validate_report_line(&mut vec![1, 12, 3, 4]), false);
    // # ascending: unsafe 2nd line item direction
    assert_eq!(validate_report_line(&mut vec![12, 11, 13, 14]), false);
    // # descending: unsafe 2nd line item diff
    assert_eq!(validate_report_line(&mut vec![21, 12, 19, 16]), false);
    // # descending: unsafe 2nd line item direction
    assert_eq!(validate_report_line(&mut vec![31, 32, 29, 27]), false);

    // # ascending: unsafe middle line item diff
    assert_eq!(validate_report_line(&mut vec![1, 3, 6, 10, 9, 10, 13, 16, 19]), false);
    // # ascending: unsafe middle line item direction
    assert_eq!(validate_report_line(&mut vec![1, 3, 6, 8, 7, 10, 13, 16, 19]), false);
    // # descending: unsafe middle line item diff
    assert_eq!(validate_report_line(&mut vec![19, 16, 13, 10, 9, 10, 6, 3, 1]), false);
    // # descending: unsafe middle line item direction
    assert_eq!(validate_report_line(&mut vec![19, 16, 13, 10, 7, 8, 6, 3, 1]), false);

    // # ascending: unsafe pre-last line item diff
    assert_eq!(validate_report_line(&mut vec![1, 2, 13, 5]), false);
    // # ascending: unsafe pre-last line item direction
    assert_eq!(validate_report_line(&mut vec![1, 2, 1, 4]), false);
    // # descending: unsafe pre-last line item diff
    assert_eq!(validate_report_line(&mut vec![11, 8, 3, 5]), false);
    // # descending: unsafe pre-last line item direction
    assert_eq!(validate_report_line(&mut vec![11, 8, 10, 5]), false);

    // # ascending: unsafe last line item diff
    assert_eq!(validate_report_line(&mut vec![1, 2, 3, 14]), false);
    // # ascending: unsafe last line item direction
    assert_eq!(validate_report_line(&mut vec![1, 2, 3, 1]), false);
    // # descending: unsafe last line item diff
    assert_eq!(validate_report_line(&mut vec![21, 20, 17, 4]), false);
    // # descending: unsafe last line item direction
    assert_eq!(validate_report_line(&mut vec![21, 20, 17, 24]), false);
  }

  #[test]
  fn it_validates_report_lines_with_dampener() {
    // # ascending: safe line items
    assert_eq!(validate_report_line_with_dampener(&mut vec![1, 2, 3, 4]), true);
    assert_eq!(validate_report_line_with_dampener(&mut vec![1, 4, 7, 10]), true);
    // # descending: safe line items
    assert_eq!(validate_report_line_with_dampener(&mut vec![4, 3, 2, 1]), true);
    assert_eq!(validate_report_line_with_dampener(&mut vec![10, 7, 4, 1]), true);

    // # ascending: safe first line item diff
    assert_eq!(validate_report_line_with_dampener(&mut vec![1, 12, 13, 14, 16]), true);
    // # ascending: safe first line item sets descending direction
    assert_eq!(validate_report_line_with_dampener(&mut vec![21, 12, 13, 14, 16]), true);
    // # descending: safe first line item diff
    assert_eq!(validate_report_line_with_dampener(&mut vec![31, 16, 14, 13, 12]), true);
    // # descending: safe first line item sets ascending direction
    assert_eq!(validate_report_line_with_dampener(&mut vec![11, 16, 14, 11, 9]), true);

    // # ascending: safe 2nd line item diff
    assert_eq!(validate_report_line_with_dampener(&mut vec![11, 22, 13, 14, 17]), true);
    // # ascending: safe 2nd line item direction
    assert_eq!(validate_report_line_with_dampener(&mut vec![12, 11, 13, 14, 17]), true);
    // # descending: safe 2nd line item diff
    assert_eq!(validate_report_line_with_dampener(&mut vec![21, 12, 19, 17, 14]), true);
    // # descending: safe 2nd line item direction
    assert_eq!(validate_report_line_with_dampener(&mut vec![21, 22, 19, 17, 14]), true);

    // # ascending: unsafe 2nd line item diff
    assert_eq!(validate_report_line_with_dampener(&mut vec![11, 22, 13, 14, 20]), false);
    // # ascending: unsafe 2nd line item direction
    assert_eq!(validate_report_line_with_dampener(&mut vec![12, 11, 13, 14, 10]), false);
    // # ascending: unsafe 2nd line item combo
    assert_eq!(validate_report_line_with_dampener(&mut vec![12, 11, 13, 14, 20]), false);
    // # descending: unsafe 2nd line item diff
    assert_eq!(validate_report_line_with_dampener(&mut vec![21, 12, 19, 15, 14]), false);
    // # descending: unsafe 2nd line item direction
    assert_eq!(validate_report_line_with_dampener(&mut vec![31, 32, 29, 30, 27]), false);
    // # descending: unsafe 2nd line item combo
    assert_eq!(validate_report_line_with_dampener(&mut vec![31, 32, 29, 25, 27]), false);

    // # ascending: safe middle line item diff
    assert_eq!(validate_report_line_with_dampener(&mut vec![1, 3, 6, 10, 9, 10, 13, 16, 19]), true);
    // # ascending: safe middle line item direction
    assert_eq!(validate_report_line_with_dampener(&mut vec![1, 3, 6, 8, 7, 10, 13, 16, 19]), true);
    // # descending: safe middle line item diff
    assert_eq!(validate_report_line_with_dampener(&mut vec![19, 16, 13, 10, 9, 10, 6, 3, 1]), true);
    // # descending: safe middle line item direction
    assert_eq!(validate_report_line_with_dampener(&mut vec![19, 16, 13, 12, 10, 7, 8, 6, 3]), true);

    // # ascending: unsafe middle line item diff
    assert_eq!(validate_report_line_with_dampener(&mut vec![1, 3, 6, 10, 8, 12, 13, 16, 19]), false);
    // # ascending: unsafe middle line item direction
    assert_eq!(validate_report_line_with_dampener(&mut vec![1, 3, 6, 9, 8, 12, 11, 14, 16]), false);
    // # ascending: unsafe middle line item combo
    assert_eq!(validate_report_line_with_dampener(&mut vec![1, 3, 6, 8, 7, 10, 14, 16, 19]), false);
    // # descending: unsafe middle line item diff
    assert_eq!(validate_report_line_with_dampener(&mut vec![19, 16, 13, 12, 8, 10, 6, 3, 1]), false);
    // # descending: unsafe middle line item direction
    assert_eq!(validate_report_line_with_dampener(&mut vec![16, 14, 11, 12, 8, 9, 6, 3, 1]), false);
    // # descending: unsafe middle line item combo
    assert_eq!(validate_report_line_with_dampener(&mut vec![19, 16, 13, 14, 12, 8, 7, 6, 3]), false);

    // # ascending: safe pre-last line item diff
    assert_eq!(validate_report_line_with_dampener(&mut vec![11, 12, 14, 23, 15]), true);
    // # ascending: safe pre-last line item direction
    assert_eq!(validate_report_line_with_dampener(&mut vec![11, 12, 14, 11, 15]), true);
    // # descending: safe pre-last line item diff
    assert_eq!(validate_report_line_with_dampener(&mut vec![13, 11, 8, 3, 5]), true);
    // # descending: safe pre-last line item direction
    assert_eq!(validate_report_line_with_dampener(&mut vec![13, 11, 8, 10, 5]), true);

    // # ascending: unsafe pre-last line item diff
    assert_eq!(validate_report_line_with_dampener(&mut vec![11, 12, 14, 23, 18]), false);
    // # ascending: unsafe pre-last line item direction
    assert_eq!(validate_report_line_with_dampener(&mut vec![11, 12, 14, 11, 10]), false);
    // # ascending: unsafe pre-last line item combo
    assert_eq!(validate_report_line_with_dampener(&mut vec![11, 12, 14, 11, 19]), false);
    // # descending: unsafe pre-last line item diff
    assert_eq!(validate_report_line_with_dampener(&mut vec![13, 11, 8, 3, 4]), false);
    // # descending: unsafe pre-last line item direction
    assert_eq!(validate_report_line_with_dampener(&mut vec![13, 11, 8, 10, 11]), false);
    // # descending: unsafe pre-last line item combo
    assert_eq!(validate_report_line_with_dampener(&mut vec![13, 11, 8, 10, 3]), false);

    // # ascending: safe last line item diff
    assert_eq!(validate_report_line_with_dampener(&mut vec![1, 2, 3, 14]), true);
    // # ascending: safe last line item direction
    assert_eq!(validate_report_line_with_dampener(&mut vec![1, 2, 3, 1]), true);
    // # descending: safe last line item diff
    assert_eq!(validate_report_line_with_dampener(&mut vec![21, 20, 17, 4]), true);
    // # descending: safe last line item direction
    assert_eq!(validate_report_line_with_dampener(&mut vec![21, 20, 17, 24]), true);
  }
}
