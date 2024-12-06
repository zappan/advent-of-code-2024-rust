use std::collections::HashMap;

pub mod benchmarks;

struct PartitionedUpdates {
  correct: Vec<Vec<String>>,
  incorrect: Vec<Vec<String>>,
}

fn get_middle_element(update: &Vec<String>) -> usize {
  let len = update.len();
  assert!(len % 2 != 0, "CRITICAL: to get a middle element, a vector must have odd number of elements");

  let mid_el_idx = len / 2;
  let mid_el = &update[mid_el_idx];
  mid_el.parse::<usize>().expect("List of updates must contain numbers")
}

fn correct_update_order(rules: &HashMap<&str, Vec<&str>>, update_order: &Vec<String>) -> Vec<String> {
  let mut order_corrected: Vec<String> = vec![];

  let mut update_order_reversed = update_order.into_iter().rev().map(|p| p.as_str()).collect::<Vec<_>>();
  while let Some((page, earlier_pages)) = update_order_reversed.split_first() {
    let mut earlier_pages = earlier_pages.to_vec();

    if let Some(pages_after_rule) = rules.get(page) {
      if let Some((idx, &page_to_move_behind)) = earlier_pages
        .iter()
        .enumerate()
        .find(|(_, ep)| pages_after_rule.contains(ep))
      {
        earlier_pages.remove(idx);
        earlier_pages.splice(0..0, [page_to_move_behind, page]);
        update_order_reversed = earlier_pages;
        continue;
      }
    }

    // ## Page is in correct order, move on to the next
    order_corrected.push(page.to_string());
    update_order_reversed = earlier_pages
  }

  order_corrected.into_iter().rev().collect::<Vec<String>>()
}

fn validate_update_order(rules: &HashMap<&str, Vec<&str>>, update_order: &Vec<String>) -> bool {
  let mut update_order_reversed = update_order.into_iter().rev().map(|p| p.as_str()).collect::<Vec<_>>();
  while let Some((page, earlier_pages)) = update_order_reversed.split_first() {
    let is_incorrect_order = match rules.get(page) {
      Some(pages_after_rule) => earlier_pages.iter().any(|ep| pages_after_rule.contains(&ep)),
      None => false,
    };

    if is_incorrect_order {
      return false;
    }

    update_order_reversed = earlier_pages.to_vec();
  }

  return true;
}

fn split_updates_by_correctness(rules: &HashMap<&str, Vec<&str>>, updates: &Vec<Vec<&str>>) -> PartitionedUpdates {
  let (correct, incorrect) = updates
    .into_iter()
    .map(|u| u.into_iter().map(|u| u.to_string()).collect::<Vec<String>>())
    .partition(|u| validate_update_order(rules, &u));
  PartitionedUpdates { correct, incorrect }
}

fn get_correct_updates(rules: &HashMap<&str, Vec<&str>>, updates: &Vec<Vec<&str>>) -> Vec<Vec<String>> {
  split_updates_by_correctness(rules, updates).correct
}

fn get_incorrect_updates(rules: &HashMap<&str, Vec<&str>>, updates: &Vec<Vec<&str>>) -> Vec<Vec<String>> {
  split_updates_by_correctness(rules, updates).incorrect
}

fn parse_input(input: &str) -> (HashMap<&str, Vec<&str>>, Vec<Vec<&str>>) {
  let (rules_input, updated_pages_input) = input.split_once("\n\n").unwrap();

  let rules = rules_input
    .split("\n")
    .filter(|x| !x.is_empty())
    .map(|line| line.split_once("|").unwrap())
    .into_iter()
    .fold(HashMap::new(), |mut map: HashMap<&str, Vec<&str>>, (k, v)| {
      match map.contains_key(k) {
        true => drop(map.get_mut(k).unwrap().push(v)),
        false => drop(map.insert(k, vec![v])),
      }
      return map;
    });

  let updated_pages = updated_pages_input
    .lines()
    .map(|l| l.split(",").collect::<Vec<&str>>())
    .collect::<Vec<Vec<&str>>>();

  (rules, updated_pages)
}

pub fn part1(input: &str) -> usize {
  let (rules, updates) = parse_input(input);
  let result = get_correct_updates(&rules, &updates)
    .into_iter()
    .map(|u| get_middle_element(&u))
    .sum();
  result
}

pub fn part2(input: &str) -> usize {
  let (rules, updates) = parse_input(input);
  let result = get_incorrect_updates(&rules, &updates)
    .into_iter()
    .map(|u| correct_update_order(&rules, &u))
    .map(|u| get_middle_element(&u))
    .sum();
  result
}
