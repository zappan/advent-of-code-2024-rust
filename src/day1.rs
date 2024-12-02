use std::collections::HashMap;

pub mod benchmarks;

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
  let mut list1: Vec<u32> = Vec::new();
  let mut list2: Vec<u32> = Vec::new();

  input
    .split("\n")
    .filter(|x| !x.is_empty())
    .map(|line| {
      line
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
    })
    .for_each(|line_item| {
      list1.push(line_item[0]);
      list2.push(line_item[1]);
    });
  (list1, list2)
}

fn calc_total_distance(mut list1: Vec<u32>, mut list2: Vec<u32>) -> u32 {
  list1.sort();
  list2.sort();

  let range_iter = 0..list1.len();
  let total_dist = range_iter.fold(0, |acc, i| {
    let (i1, i2) = (list1[i], list2[i]);
    let dist = match i1.cmp(&i2) {
      std::cmp::Ordering::Greater => i1 - i2,
      std::cmp::Ordering::Less => i2 - i1,
      std::cmp::Ordering::Equal => 0,
    };
    // println!("{}: {} {} {}", i, list1[i], list2[i], dist);
    return acc + dist;
  });

  total_dist
}

fn calc_similarity_score(list1: &Vec<u32>, list2: &Vec<u32>) -> u32 {
  list1.iter().fold(0, |acc, item| {
    let count = list2.iter().filter(|i| item == *i).count() as u32;
    let similarity = item * count;
    // println!("{} => {} : {}", item, count, similarity);
    return acc + similarity;
  })
}

fn calc_similarity_score_fast(list1: &Vec<u32>, list2: &Vec<u32>) -> u32 {
  let mut counts: HashMap<u32, u32> = HashMap::new();
  list1.iter().for_each(|list_item| {
    counts.insert(*list_item, 0);
  });

  list2.iter().for_each(|list_item| {
    counts.entry(*list_item).and_modify(|count| *count += 1);
  });

  counts.into_iter().fold(0, |acc, (item, count)| {
    let similarity = item * count;
    return acc + similarity;
  })
}

pub fn part1(input: &str) -> u32 {
  let (list1, list2) = parse_input(input);
  calc_total_distance(list1, list2)
}

pub fn part2(input: &str) -> u32 {
  let (list1, list2) = parse_input(input);
  calc_similarity_score_fast(&list1, &list2)
}
