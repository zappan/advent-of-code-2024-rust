pub mod benchmarks;

fn calc_checksum(blocks: Vec<String>) -> usize {
  blocks
    .into_iter()
    .enumerate()
    .filter(|(_, file_id)| file_id != ".")
    .map(|(idx, file_id)| {
      let file_id = file_id.parse::<usize>().unwrap();
      return idx * file_id;
    })
    .sum::<usize>()
}

fn rearrange_whole_file_blocks(mut blocks: Vec<String>) -> Vec<String> {
  let upper_bound = blocks.len();

  let mut rev_iter = (0..upper_bound).rev();
  while let Some(rev_idx) = rev_iter.next() {
    let rev_file_id = &blocks[rev_idx];

    if rev_file_id == "." {
      continue;
    }

    let mut rev_file_block_idx = rev_idx;
    let mut rev_file_block_indices = vec![];

    while rev_file_id == &blocks[rev_file_block_idx] {
      rev_file_block_indices.push(rev_file_block_idx);
      if rev_file_block_idx == 0 {
        break;
      }
      rev_file_block_idx -= 1;
    }
    let rev_file_block_size = rev_file_block_indices.len();

    let mut empty_space_iter = 0..=rev_file_block_idx;
    while let Some(idx) = empty_space_iter.next() {
      let file_id = &blocks[idx];
      if file_id != "." {
        continue;
      }

      let mut empty_space_block_idx = idx;
      let mut empty_space_block_indices = vec![];
      while file_id == &blocks[empty_space_block_idx] {
        empty_space_block_indices.push(empty_space_block_idx);
        if empty_space_block_idx >= rev_file_block_idx {
          break;
        }
        empty_space_block_idx += 1;
      }
      let empty_space_block_size = empty_space_block_indices.len();

      if empty_space_block_size >= rev_file_block_size {
        for i in 0..rev_file_block_size {
          blocks.swap(rev_file_block_indices[i], empty_space_block_indices[i]);
        }

        break;
      }
    }

    let should_skip = rev_file_block_size > 1;
    if should_skip {
      let remaining_block_skip = rev_file_block_size - 2; // -1 because nth-index starts from zero, and another -1 because one has been consumed,
      rev_iter.nth(remaining_block_skip);
    }
  }

  return blocks;
}

fn rearrange_partial_file_blocks(mut blocks: Vec<String>) -> Vec<String> {
  let mut upper_bound = blocks.len();
  let mut iters_crossed = false;
  for idx in 0..upper_bound {
    let file_id = &blocks[idx];
    if file_id == "." {
      for rev_idx in (0..upper_bound).rev() {
        iters_crossed = rev_idx <= idx;
        if iters_crossed {
          break;
        }

        let rev_file_id = &blocks[rev_idx];
        if rev_file_id != "." {
          blocks.swap(idx, rev_idx);
          upper_bound = rev_idx;
          break;
        }
      }
    }

    if iters_crossed {
      break;
    }
  }

  return blocks;
}

fn parse_input(input: &str) -> Vec<String> {
  let mut parsed_input: Vec<String> = vec![];

  input.trim_end().chars().enumerate().for_each(|(i, c)| {
    let is_file = i % 2 == 0;

    let file_id = match is_file {
      true => (i / 2).to_string(),
      false => ".".to_string(),
    };

    let block_len = c.to_digit(10).unwrap();
    (0..block_len).for_each(|_| parsed_input.push(file_id.clone()))
  });

  return parsed_input;
}

pub fn part1(input: &str) -> usize {
  let blocks = parse_input(input);
  let rearranged = rearrange_partial_file_blocks(blocks);
  let checksum = calc_checksum(rearranged);
  checksum
}

pub fn part2(input: &str) -> usize {
  let blocks = parse_input(input);
  let rearranged = rearrange_whole_file_blocks(blocks);
  let checksum = calc_checksum(rearranged);
  checksum
}
