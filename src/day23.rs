use std::collections::BTreeSet;
use std::collections::HashSet;
use std::hash::Hash;
use std::hash::Hasher;

use itertools::Itertools;

pub mod benchmarks;

#[derive(Debug)]
struct Conn(String, String);

#[derive(Debug, Eq)]
struct ThreeConnSet(String, String, String);

impl PartialEq for ThreeConnSet {
  fn eq(&self, other: &ThreeConnSet) -> bool {
    let self_items: Vec<&String> = [&self.0, &self.1, &self.2].into_iter().sorted().collect();
    let other_items: Vec<&String> = [&other.0, &other.1, &other.2].into_iter().sorted().collect();
    self_items == other_items
  }
}

impl Hash for ThreeConnSet {
  fn hash<H>(&self, state: &mut H)
  where
    H: Hasher,
  {
    let items: Vec<&String> = [&self.0, &self.1, &self.2].into_iter().sorted().collect();
    for item in items {
      item.hash(state);
    }
  }
}

// Define a type alias for HashSet<String>
type ConnSet = BTreeSet<String>;

fn _print_conn_sets(conn_sets: &Vec<ConnSet>) {
  println!("###############################################################################");
  println!("CONNECTION SETS:");
  conn_sets
    .iter()
    .sorted_by(|a, b| Ord::cmp(&a.len(), &b.len()))
    .for_each(|cs| println!("{:?}", cs));
  println!("###############################################################################");
}

fn parse_input(input: &str) -> Vec<Conn> {
  input
    .lines()
    .map(|l| l.split_once('-').unwrap())
    .map(|(l, r)| Conn(l.to_string(), r.to_string()))
    .collect()
}

fn extract_computers(conns: &Vec<Conn>) -> Vec<String> {
  conns
    .into_iter()
    .flat_map(|conn| vec![conn.0.clone(), conn.1.clone()])
    .unique()
    .sorted()
    .collect()
}

fn find_three_conn_sets(computers: &Vec<String>, connections: &Vec<Conn>) -> HashSet<ThreeConnSet> {
  let mut three_sets: HashSet<ThreeConnSet> = HashSet::new();

  computers.iter().for_each(|comp1| {
    let comp2s: Vec<String> = connections
      .into_iter()
      .filter(|conn| [&conn.0, &conn.1].contains(&comp1))
      .map(|conn| {
        if conn.0 == *comp1 {
          conn.1.clone()
        } else {
          conn.0.clone()
        }
      })
      .collect();

    comp2s.iter().for_each(|comp2| {
      comp2s.iter().for_each(|comp3| {
        if connections.into_iter().any(|conn| {
          if conn.0 == **comp3 {
            conn.1 == *comp2
          } else if conn.1 == **comp3 {
            conn.0 == *comp2
          } else {
            false
          }
        }) {
          three_sets.insert(ThreeConnSet(comp1.to_string(), comp2.to_string(), comp3.to_string()));
        }
      });
    });
  });

  three_sets
}

fn find_sets_starting_with_t(three_sets: &HashSet<ThreeConnSet>) -> Vec<&ThreeConnSet> {
  three_sets
    .iter()
    .filter(|s| s.0.starts_with("t") || s.1.starts_with("t") || s.2.starts_with("t"))
    .collect()
}

fn is_connected_to_all_other_comps(
  conn_set_candidate: &String,
  target_comps: &Vec<&String>,
  connections: &Vec<Conn>,
) -> bool {
  // ## take conn_set_candidate and find if it's connected with all of the target_comps
  let candidate_all_conns: Vec<&Conn> = connections
    .iter()
    .filter(|conn| [&conn.0, &conn.1].contains(&conn_set_candidate))
    .collect();

  let candidate_connects_all_targets = target_comps.iter().all(|comp| {
    candidate_all_conns
      .iter()
      .any(|conn| [&conn.0, &conn.1].contains(&comp))
  });

  candidate_connects_all_targets
}

fn process_connection_set_candidate(candidate: &String, conn_set: &mut ConnSet, connections: &Vec<Conn>) {
  let conn_set_comps: Vec<&String> = conn_set.iter().map(|comp| comp).sorted().collect();
  if is_connected_to_all_other_comps(candidate, &conn_set_comps, connections) {
    conn_set.insert(candidate.to_string());
  }
}

fn process_candidate(
  candidate: &String,
  existing_set_member: &String,
  conn_sets: &mut Vec<ConnSet>,
  connections: &Vec<Conn>,
) {
  for conn_set in conn_sets {
    if conn_set.contains(existing_set_member) {
      process_connection_set_candidate(candidate, conn_set, connections);
    }
  }
}

fn find_all_connection_sets(_computers: &Vec<String>, connections: &Vec<Conn>) -> Vec<ConnSet> {
  let mut conn_sets: Vec<ConnSet> = vec![];

  connections.iter().for_each(|conn| {
    let (comp1, comp2) = (conn.0.clone(), conn.1.clone());

    process_candidate(&comp2, &comp1, &mut conn_sets, connections);
    process_candidate(&comp1, &comp2, &mut conn_sets, connections);
    conn_sets.push(ConnSet::from([comp1, comp2]));
  });

  return conn_sets.into_iter().unique().collect();
}

fn find_largest_conn_set(conn_sets: &Vec<ConnSet>) -> &ConnSet {
  conn_sets
    .into_iter()
    .reduce(|largest_till_now, cs| match cs.len() > largest_till_now.len() {
      true => cs,
      false => largest_till_now,
    })
    .unwrap()
}

fn calculate_password(conn_set: &ConnSet) -> String {
  conn_set.into_iter().sorted().join(",")
}

pub fn part1(input: &str) -> usize {
  let connections = parse_input(input);
  let computers = extract_computers(&connections);
  let three_sets = find_three_conn_sets(&computers, &connections);
  let sets_starting_with_t = find_sets_starting_with_t(&three_sets);
  let result = sets_starting_with_t.len();
  result
}

pub fn part2(input: &str) -> String {
  let connections = parse_input(input);
  let computers = extract_computers(&connections);
  let all_connection_sets: Vec<ConnSet> = find_all_connection_sets(&computers, &connections);

  // _print_conn_sets(&all_connection_sets);

  let largest_set = find_largest_conn_set(&all_connection_sets);
  let result = calculate_password(&largest_set);
  result
}
