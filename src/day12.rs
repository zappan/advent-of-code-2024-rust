use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug, Eq, Hash, PartialEq)]
enum Border {
  N,
  E,
  S,
  W,
}

#[derive(Debug)]
struct Coords {
  x: usize,
  y: usize,
}

impl Coords {
  fn new(x: usize, y: usize) -> Self {
    Self { x, y }
  }
}

#[derive(Debug)]
struct Plot {
  plant: char,
  coords: Coords,
  fence: HashMap<Border, bool>,
  area: Option<u16>,
}

impl Plot {
  fn new(plant: char, row_idx: usize, col_idx: usize) -> Self {
    Self {
      plant,
      coords: Coords::new(row_idx, col_idx),
      fence: HashMap::from([
        (Border::N, true),
        (Border::E, true),
        (Border::S, true),
        (Border::W, true),
      ]),
      area: None,
    }
  }
}

#[derive(Debug)]
struct Garden {
  size: Coords,
  plots: Vec<Vec<Plot>>,
}

impl Garden {
  fn new(plots: Vec<Vec<Plot>>) -> Self {
    let (x, y) = (plots.len(), plots[0].len());

    Self {
      size: Coords { x, y },
      plots,
    }
  }
}

fn have_same_plants(plots: &Vec<Vec<Plot>>, coords1: &Coords, coords2: &Coords) -> bool {
  plots[coords1.x][coords1.y].plant == plots[coords2.x][coords2.y].plant
}

fn assign_fences_and_area(plots: &mut Vec<Vec<Plot>>, plot_coords: &Coords, area_id: u16, garden_size: &Coords) {
  let (x, y) = (plot_coords.x, plot_coords.y);
  let has_north = x > 0;
  let has_south = x < garden_size.x - 1;
  let has_west = y > 0;
  let has_east = y < garden_size.y - 1;

  let plot = &mut plots[x][y];

  if plot.area.is_some() {
    return;
  }

  plot.area = Some(area_id);

  if has_north {
    let nc = Coords { x: x - 1, y };
    let same_area = have_same_plants(plots, plot_coords, &nc);
    let plot = &mut plots[x][y];
    *plot.fence.get_mut(&Border::N).unwrap() = !same_area;

    if same_area {
      assign_fences_and_area(plots, &nc, area_id, garden_size);
    }
  }

  if has_west {
    let wc = Coords { x, y: y - 1 };
    let same_area = have_same_plants(plots, plot_coords, &wc);
    let plot = &mut plots[x][y];
    *plot.fence.get_mut(&Border::W).unwrap() = !same_area;

    if same_area {
      assign_fences_and_area(plots, &wc, area_id, garden_size);
    }
  }

  if has_south {
    let sc = Coords { x: x + 1, y };
    let same_area = have_same_plants(plots, plot_coords, &sc);
    let plot = &mut plots[x][y];
    *plot.fence.get_mut(&Border::S).unwrap() = !same_area;

    if same_area {
      assign_fences_and_area(plots, &sc, area_id, garden_size);
    }
  }

  if has_east {
    let ec = Coords { x, y: y + 1 };
    let same_area = have_same_plants(plots, plot_coords, &ec);
    let plot = &mut plots[x][y];
    *plot.fence.get_mut(&Border::E).unwrap() = !same_area;

    if same_area {
      assign_fences_and_area(plots, &ec, area_id, garden_size);
    }
  }
}

fn calc_fences_and_areas(garden: &mut Garden) {
  let plots = &mut garden.plots;
  let mut area_id: u16 = 0;

  for x in 0..garden.size.x {
    for y in 0..garden.size.y {
      let plot = &plots[x][y];

      if plot.area.is_none() {
        area_id += 1;
        let plot_coords = Coords { x, y };
        assign_fences_and_area(plots, &plot_coords, area_id, &garden.size);
      }
    }
  }
}

fn get_regions(garden: &Garden) -> HashMap<u16, Vec<&Plot>> {
  let mut regions: HashMap<u16, Vec<&Plot>> = HashMap::new();
  garden
    .plots
    .iter()
    .flat_map(|plots| plots.into_iter().map(|p| p))
    .for_each(|p| {
      if let Some(region) = regions.get_mut(&p.area.unwrap()) {
        region.push(p);
      } else {
        regions.insert(p.area.unwrap(), vec![p]);
      }
    });
  regions
}

fn calculate_price_by_perimeter(regions: &HashMap<u16, Vec<&Plot>>) -> usize {
  regions
    .into_iter()
    .map(|(_region, plots)| {
      (
        plots.len(),
        plots
          .iter()
          .map(|p| p.fence.values().filter(|&&v| v).count())
          .sum::<usize>(),
      )
    })
    .map(|(area, perimeter)| area * perimeter)
    .sum()
}

fn _calculate_price_by_sides_count(regions: &HashMap<u16, Vec<&Plot>>) -> usize {
  regions
    .into_iter()
    // .inspect(|r| println!("\nRegion: {:?}", r))
    // .inspect(|(_r, plots)| plots.into_iter().for_each(|&p| println!("Fence: {:?}", p.fence)))
    .map(|(_region, plots)| {
      (
        // region,
        //plots.iter().map(|p| p.).collect::<Vec<_>>(),
        plots.len(),
        plots
          .iter()
          .map(|p| p.fence.values().filter(|&&v| v).count())
          // .map(|p| &p.fence)
          // .map(|fence| fence.values().filter(|&&v| v).count())
          // .collect::<Vec<_>>(),
          .sum::<usize>(),
      )
    })
    .map(|(area, perimeter)| area * perimeter)
    .sum()
}

fn parse_input(input: &str) -> Garden {
  Garden::new(
    input
      .lines()
      .enumerate()
      .map(|(row_idx, line)| {
        line
          .chars()
          .enumerate()
          .map(|(col_idx, plant)| Plot::new(plant, row_idx, col_idx))
          .collect::<Vec<Plot>>()
      })
      .collect::<Vec<Vec<Plot>>>(),
  )
}

pub fn part1(input: &str) -> usize {
  let mut garden = parse_input(input);
  calc_fences_and_areas(&mut garden);
  let regions = get_regions(&garden);
  let price = calculate_price_by_perimeter(&regions);
  price

  // println!("Price: {:?}", price);

  // input.len()
}
