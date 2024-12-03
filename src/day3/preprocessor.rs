#[derive(Debug)]
enum PreprocessorState {
  Enabled,
  Disabled,
}

pub fn preprocess(input: &str) -> String {
  let mut result = vec![];

  let mut on_off: Vec<(usize, &str)> = vec![];
  let mut do_s: Vec<(usize, &str)> = input.match_indices("do()").collect();
  let mut dont_s: Vec<(usize, &str)> = input.match_indices("don't()").collect();

  on_off.append(&mut do_s);
  on_off.append(&mut dont_s);
  on_off.sort_by_key(|k| k.0);

  let mut preprocessor_state = PreprocessorState::Enabled; // initial state is ENABLED
  let mut start_index = 0;
  let mut end_index = 0;
  let mut start_indices = vec![start_index];
  let mut end_indices = vec![];
  on_off.into_iter().for_each(|(k, v)| match preprocessor_state {
    PreprocessorState::Disabled => {
      if v == "do()" {
        preprocessor_state = PreprocessorState::Enabled;
        start_index = k + v.len();
        start_indices.push(start_index);
      }
    }
    PreprocessorState::Enabled => {
      if v == "don't()" {
        preprocessor_state = PreprocessorState::Disabled;
        end_index = k;
        end_indices.push(end_index);
      }
    }
  });

  if matches!(preprocessor_state, PreprocessorState::Enabled) {
    end_index = input.len();
    end_indices.push(end_index);
  }

  assert_eq!(start_indices.len(), end_indices.len(), "CRITICAL: start-end indices length mismatch");

  for i in 0..start_indices.len() {
    result.push(&input[start_indices[i]..end_indices[i]]);
  }

  return result.into_iter().collect();
}
