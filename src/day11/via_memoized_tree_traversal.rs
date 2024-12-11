use std::collections::HashMap;

fn parse_stone(stone: &str) -> String {
  stone.parse::<usize>().unwrap().to_string()
}

fn apply_rules(stone: &String) -> Vec<String> {
  let engrave_len = stone.len();
  if stone == "0" {
    let new_stone = String::from("1");
    vec![new_stone]
  } else if engrave_len % 2 == 0 {
    let split_at = engrave_len / 2;
    let (stone1, stone2) = stone.split_at(split_at);
    vec![parse_stone(stone1), parse_stone(stone2)]
  } else {
    let stone_nr: usize = stone.parse().unwrap();
    let new_stone = (stone_nr * 2024).to_string();
    vec![new_stone]
  }
}

fn blink(stones: Vec<String>, stone_count_memo: &mut HashMap<(String, u8), usize>, remaining_blinks: u8) -> usize {
  if remaining_blinks == 0 {
    return stones.len();
  }

  stones
    .into_iter()
    .map(|stone| {
      if let Some(&count) = stone_count_memo.get(&(stone.clone(), remaining_blinks)) {
        return count;
      } else {
        let count = blink(apply_rules(&stone), stone_count_memo, remaining_blinks - 1);
        stone_count_memo.insert((stone, remaining_blinks), count);
        return count;
      }
    })
    .sum()
}

fn parse_input(input: &str) -> Vec<String> {
  input.trim().split(" ").map(|s| s.to_string()).collect()
}

pub fn part1(input: &str) -> usize {
  let blink_count = 25;
  let mut stone_count_memo: HashMap<(String, u8), usize> = HashMap::new();
  let stones = parse_input(input);
  let stones_len = blink(stones, &mut stone_count_memo, blink_count);
  stones_len
}

pub fn part2(input: &str) -> usize {
  let blink_count = 75;
  let mut stone_count_memo: HashMap<(String, u8), usize> = HashMap::new();
  let stones = parse_input(input);
  let stones_len = blink(stones, &mut stone_count_memo, blink_count);
  stones_len
}
