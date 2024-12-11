use itertools::Itertools;

#[derive(Debug)]
struct MapSize {
  x: usize,
  y: usize,
}

fn get_trail_neighbouring_indices(tth_idx: usize, size: &MapSize) -> Vec<usize> {
  let out_of_bounds_idx = size.x * size.y;

  let mut neighbours = vec![];
  // north
  if tth_idx >= size.x {
    neighbours.push(tth_idx - size.x);
  }
  // west
  if tth_idx % size.x != 0 {
    neighbours.push(tth_idx - 1);
  }
  // south
  if tth_idx + size.x < out_of_bounds_idx {
    neighbours.push(tth_idx + size.x);
  }
  // east
  if tth_idx % size.x < size.x - 1 {
    neighbours.push(tth_idx + 1);
  }
  return neighbours;
}

fn find_next_trail_traversing_indices(topo_map: &Vec<u32>, map_size: &MapSize, tth_idx: usize) -> Vec<usize> {
  let current_field = topo_map[tth_idx];
  let neighbouring_indices = get_trail_neighbouring_indices(tth_idx, &map_size);

  neighbouring_indices
    .into_iter()
    .map(|idx| (idx, topo_map[idx]))
    .filter(|&(_, map_field)| map_field == current_field + 1)
    .map(|(idx, _)| idx)
    .collect::<Vec<usize>>()
}

fn traverse_to_trailends(topo_map: &Vec<u32>, map_size: &MapSize, tth_idx: usize) -> Vec<(usize, u32)> {
  let tth = topo_map[tth_idx];

  if tth == 9 {
    return vec![(tth_idx, tth)];
  }

  let next_trail_traversing_indices = find_next_trail_traversing_indices(&topo_map, &map_size, tth_idx);

  next_trail_traversing_indices
    .into_iter()
    .flat_map(|next_tth_idx| traverse_to_trailends(topo_map, map_size, next_tth_idx))
    .collect::<Vec<_>>()
}

fn find_trails(topo_map: Vec<u32>, map_size: MapSize) -> Vec<((usize, u32), Vec<(usize, u32)>)> {
  let trailheads: Vec<(usize, u32)> = topo_map
    .clone()
    .into_iter()
    .enumerate()
    .filter(|&(_, f)| f == 0)
    .collect();

  let trails = trailheads
    .into_iter()
    .map(|(tth_idx, _)| {
      let next_trail_fragments = traverse_to_trailends(&topo_map, &map_size, tth_idx);
      return ((tth_idx, topo_map[tth_idx]), next_trail_fragments);
    })
    .collect::<Vec<((usize, u32), Vec<(usize, u32)>)>>();

  trails
}

fn get_trailheads_scores(trails: Vec<((usize, u32), Vec<(usize, u32)>)>) -> Vec<((usize, u32), usize)> {
  trails
    .into_iter()
    .map(|(trailhead, trailends)| (trailhead, trailends.into_iter().unique().collect::<Vec<_>>()))
    .map(|(trailhead, trailends)| (trailhead, trailends.len()))
    .collect::<Vec<((usize, u32), usize)>>()
}

fn sum_trailheads_scores(trailheads_scores: Vec<((usize, u32), usize)>) -> usize {
  trailheads_scores.into_iter().map(|(_, score)| score).sum()
}

fn get_trailheads_ratings(trails: Vec<((usize, u32), Vec<(usize, u32)>)>) -> Vec<((usize, u32), usize)> {
  trails
    .into_iter()
    .map(|(trailhead, trailends)| (trailhead, trailends.len()))
    .collect::<Vec<((usize, u32), usize)>>()
}

fn sum_trailheads_ratings(trailheads_ratings: Vec<((usize, u32), usize)>) -> usize {
  trailheads_ratings.into_iter().map(|(_, score)| score).sum()
}

fn parse_input(input: &str) -> (MapSize, Vec<u32>) {
  let rows_iter = input.lines().filter(|x| !x.is_empty());

  let map_size = MapSize {
    x: rows_iter.clone().nth(1).unwrap().len(),
    y: rows_iter.clone().count(),
  };

  let topo_map = input
    .lines()
    .filter(|l| !l.is_empty())
    .flat_map(|l| {
      l.chars()
        .map(|c| {
          let d = c.to_digit(10).unwrap();
          return d;
        })
        .collect::<Vec<u32>>()
    })
    .collect::<Vec<u32>>();

  (map_size, topo_map)
}

pub fn part1(input: &str) -> usize {
  let (map_size, topo_map) = parse_input(input);
  let trails = find_trails(topo_map, map_size);
  let trailheads_scores = get_trailheads_scores(trails);
  let result = sum_trailheads_scores(trailheads_scores);
  result
}

pub fn part2(input: &str) -> usize {
  let (map_size, topo_map) = parse_input(input);
  let trails = find_trails(topo_map, map_size);
  let trailheads_ratings = get_trailheads_ratings(trails);
  let result = sum_trailheads_ratings(trailheads_ratings);
  result
}
