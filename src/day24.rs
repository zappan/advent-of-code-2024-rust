use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

pub mod benchmarks;

#[derive(Debug)]
enum GateType {
  AND,
  OR,
  XOR,
}

#[derive(Debug)]
enum ParseGateTypeError {
  BadType,
}

impl FromStr for GateType {
  type Err = ParseGateTypeError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "AND" => Ok(GateType::AND),
      "OR" => Ok(GateType::OR),
      "XOR" => Ok(GateType::XOR),
      _ => Err(ParseGateTypeError::BadType),
    }
  }
}

#[derive(Debug)]
struct Gate {
  in1: String,
  in2: String,
  gate: GateType,
}

fn parse_input(input: &str) -> (HashMap<String, bool>, HashMap<String, Gate>) {
  let (init_wires, gate_conns) = input.trim().split_once("\n\n").unwrap();

  let mut wires: HashMap<String, bool> = HashMap::new();
  init_wires.lines().for_each(|l| {
    let (k, v) = l.split_once(": ").unwrap();
    let val = match v {
      "1" => true,
      _ => false,
    };
    wires.insert(k.to_string(), val);
  });

  let capture_regex = Regex::new(r"([a-z0-9]{3}) (AND|OR|XOR) ([a-z0-9]{3}) -> ([a-z0-9]{3})").unwrap();
  let mut gates: HashMap<String, Gate> = HashMap::new();
  gate_conns.lines().for_each(|l| {
    let capture = capture_regex.captures(&l).unwrap();
    let in1 = capture.get(1).unwrap().as_str().to_string();
    let gate = capture.get(2).unwrap().as_str().parse::<GateType>().unwrap();
    let in2 = capture.get(3).unwrap().as_str().to_string();
    let key = capture.get(4).unwrap().as_str().to_string();

    let gate = Gate { in1, in2, gate };
    gates.insert(key, gate);
  });

  (wires, gates)
}

fn resolve_wire_value(wire_key: &String, gates: &HashMap<String, Gate>, wires: &mut HashMap<String, bool>) -> bool {
  match wires.get(wire_key) {
    Some(&val) => val,
    None => resolve_gate(wire_key, gates, wires),
  }
}

fn resolve_gate(wire_key: &String, gates: &HashMap<String, Gate>, wires: &mut HashMap<String, bool>) -> bool {
  match wires.get(wire_key) {
    Some(&val) => val,
    None => {
      let gate: &Gate = gates.get(wire_key).unwrap();
      let in1 = resolve_wire_value(&gate.in1, gates, wires);
      let in2 = resolve_wire_value(&gate.in2, gates, wires);
      let out = match gate.gate {
        GateType::AND => in1 & in2,
        GateType::OR => in1 | in2,
        GateType::XOR => in1 ^ in2,
      };
      wires.insert(wire_key.to_string(), out);
      out
    }
  }
}

fn resolve_all_gates(gates: &mut HashMap<String, Gate>, wires: &mut HashMap<String, bool>) {
  let keys: Vec<String> = gates.keys().map(|k| k.to_string()).collect();
  keys.into_iter().for_each(|key| {
    resolve_gate(&key, gates, wires);
  });
}

fn get_end_values_keys(wires: &HashMap<String, bool>) -> Vec<String> {
  let match_regex = Regex::new(r"z[0-9]{2}").unwrap();
  wires
    .keys()
    .map(|k| k.to_string())
    .filter(|k| match_regex.is_match(k))
    .sorted()
    .rev()
    .collect()
}

fn get_end_values(wires: &HashMap<String, bool>, keys: Vec<String>) -> String {
  keys.into_iter().fold(String::new(), |mut acc, k| {
    match *wires.get(&k).unwrap() {
      true => acc.push('1'),
      false => acc.push('0'),
    };
    acc
  })
}

fn get_end_result(wires: &HashMap<String, bool>) -> usize {
  let result_keys = get_end_values_keys(wires);
  let binary_result = get_end_values(wires, result_keys.clone());
  usize::from_str_radix(binary_result.as_str(), 2).unwrap()
}

pub fn part1(input: &str) -> usize {
  let (mut wires, mut gates) = parse_input(input);
  resolve_all_gates(&mut gates, &mut wires);
  let result = get_end_result(&wires);
  result
}
