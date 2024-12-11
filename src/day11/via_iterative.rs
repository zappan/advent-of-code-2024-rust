fn parse_stone(stone: &str) -> String {
  stone.parse::<usize>().unwrap().to_string()
}

fn apply_rules(stone: String) -> Vec<String> {
  let mut result = vec![];
  let engrave_len = stone.len();
  if stone == "0" {
    let new_stone = String::from("1");
    result.push(new_stone);
  } else if engrave_len % 2 == 0 {
    let split_at = engrave_len / 2;
    let new_stones = stone.split_at(split_at);
    let stone1 = parse_stone(new_stones.0);
    let stone2 = parse_stone(new_stones.1);
    result.push(stone1);
    result.push(stone2);
  } else {
    let stone_nr: usize = stone.parse().unwrap();
    let new_stone = (stone_nr * 2024).to_string();
    result.push(new_stone);
  }
  return result;
}

fn blink(stones: Vec<String>) -> Vec<String> {
  stones.into_iter().flat_map(|s| apply_rules(s)).collect()
}

fn parse_input(input: &str) -> Vec<String> {
  input.trim().split(" ").map(|s| s.to_string()).collect()
}

pub fn part1(input: &str) -> usize {
  let blink_count = 25;
  let mut stones = parse_input(input);
  for _iter in 0..blink_count {
    stones = blink(stones);
    // // if _iter < 10 {
    // if _iter == 24 {
    //   println!("STONES at level {}:", _iter);
    //   println!("{:?}", stones);
    // }
  }
  stones.len()
}

pub fn part2(input: &str) -> usize {
  let blink_count = 75;
  let mut stones = parse_input(input);
  for _iter in 0..blink_count {
    print!(".");
    // println!("ITER {}", _iter);
    stones = blink(stones);
  }
  stones.len()
}
