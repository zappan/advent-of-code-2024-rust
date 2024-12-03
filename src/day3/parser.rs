pub trait State {
  fn is_final_state(&self) -> bool {
    false
  }

  fn get_matched_content(&self) -> &str {
    return "";
  }

  fn reset(self: Box<Self>) -> Box<dyn State> {
    Box::new(ResetState::new())
  }

  fn transition(self: Box<Self>, c: char) -> Box<dyn State>;

  fn consume(self: Box<Self>, c: char) -> Box<dyn State>;
}

// ======================================================================================

struct ResetState {
  matched_content: String,
  state_content: String,
}
struct MulState {
  matched_content: String,
  expected_content: String,
  state_content: String,
}
struct OpenParenState {
  matched_content: String,
  state_content: String,
}
struct FirstIntState {
  matched_content: String,
  state_content: String,
}
struct CommaState {
  matched_content: String,
  state_content: String,
}
struct SecondIntState {
  matched_content: String,
  state_content: String,
}
struct CloseParenState {
  matched_content: String,
  state_content: String,
}
struct MatchedState {
  matched_content: String,
}

// --------------------------------------------------------------------------------------

impl ResetState {
  fn new() -> Self {
    Self {
      matched_content: String::new(),
      state_content: String::new(),
    }
  }
}

impl MulState {
  fn new(prev_state: &ResetState, c: char) -> Self {
    Self {
      matched_content: format!("{}{}", &prev_state.matched_content, &prev_state.state_content),
      state_content: c.to_string(),
      expected_content: String::from("mul"),
    }
  }
}

impl OpenParenState {
  fn new(prev_state: &MulState) -> Self {
    Self {
      matched_content: format!("{}{}", &prev_state.matched_content, &prev_state.state_content),
      state_content: String::new(),
    }
  }
}

impl FirstIntState {
  fn new(prev_state: &OpenParenState) -> Self {
    Self {
      matched_content: format!("{}{}", &prev_state.matched_content, &prev_state.state_content),
      state_content: String::new(),
    }
  }
}

impl CommaState {
  fn new(prev_state: &FirstIntState) -> Self {
    Self {
      matched_content: format!("{}{}", &prev_state.matched_content, &prev_state.state_content),
      state_content: String::new(),
    }
  }
}

impl SecondIntState {
  fn new(prev_state: &CommaState) -> Self {
    Self {
      matched_content: format!("{}{}", &prev_state.matched_content, &prev_state.state_content),
      state_content: String::new(),
    }
  }
}

impl CloseParenState {
  fn new(prev_state: &SecondIntState) -> Self {
    Self {
      matched_content: format!("{}{}", &prev_state.matched_content, &prev_state.state_content),
      state_content: String::new(),
    }
  }
}

impl MatchedState {
  fn new(prev_state: &CloseParenState) -> Self {
    Self {
      matched_content: format!("{}{}", &prev_state.matched_content, &prev_state.state_content),
    }
  }
}

// ======================================================================================

impl State for ResetState {
  fn transition(self: Box<Self>, c: char) -> Box<dyn State> {
    Box::new(MulState::new(&self, c))
  }

  fn consume(self: Box<Self>, c: char) -> Box<(dyn State)> {
    match c {
      'm' => self.transition(c),
      _ => self,
    }
  }
}

impl State for MulState {
  fn transition(self: Box<Self>, _: char) -> Box<dyn State> {
    Box::new(OpenParenState::new(&self))
  }

  fn consume(mut self: Box<Self>, c: char) -> Box<dyn State> {
    self.state_content.push(c);

    match self.expected_content.starts_with(&self.state_content) {
      true => match self.state_content == self.expected_content {
        true => self.transition(c),
        false => self,
      },
      false => self.reset(),
    }
  }
}

impl State for OpenParenState {
  fn transition(self: Box<Self>, _: char) -> Box<dyn State> {
    Box::new(FirstIntState::new(&self))
  }

  fn consume(mut self: Box<Self>, c: char) -> Box<dyn State> {
    self.state_content.push(c);

    match c {
      '(' => self.transition(c),
      _ => self.reset(),
    }
  }
}

impl State for FirstIntState {
  fn transition(self: Box<Self>, c: char) -> Box<dyn State> {
    Box::new(CommaState::new(&self)).consume(c)
  }

  fn consume(mut self: Box<Self>, c: char) -> Box<dyn State> {
    fn verify_state_content(state_content: &String) -> bool {
      // ## verify content to be a 1-3 digit number
      match state_content.len() {
        1..=3 => state_content.parse::<u16>().is_ok(),
        _ => false,
      }
    }

    match c {
      '0'..='9' => {
        self.state_content.push(c);
        self
      }
      ',' => match verify_state_content(&self.state_content) {
        true => self.transition(c),
        false => self.reset(),
      },
      _ => self.reset(),
    }
  }
}

impl State for CommaState {
  fn transition(self: Box<Self>, _: char) -> Box<dyn State> {
    Box::new(SecondIntState::new(&self))
  }

  fn consume(mut self: Box<Self>, c: char) -> Box<dyn State> {
    self.state_content.push(c);

    match c {
      ',' => self.transition(c),
      _ => self.reset(),
    }
  }
}

impl State for SecondIntState {
  fn transition(self: Box<Self>, c: char) -> Box<dyn State> {
    Box::new(CloseParenState::new(&self)).consume(c)
  }

  fn consume(mut self: Box<Self>, c: char) -> Box<dyn State> {
    fn verify_state_content(state_content: &String) -> bool {
      // ## verify content to be a 1-3 digit number
      match state_content.len() {
        1..=3 => state_content.parse::<u16>().is_ok(),
        _ => false,
      }
    }

    match c {
      '0'..='9' => {
        self.state_content.push(c);
        self
      }
      ')' => match verify_state_content(&self.state_content) {
        true => self.transition(c),
        false => self.reset(),
      },
      _ => self.reset(),
    }
  }
}

impl State for CloseParenState {
  fn consume(mut self: Box<Self>, c: char) -> Box<dyn State> {
    self.state_content.push(c);

    match c {
      ')' => self.transition(c),
      _ => self.reset(),
    }
  }

  fn transition(self: Box<Self>, _c: char) -> Box<dyn State> {
    Box::new(MatchedState::new(&self))
  }
}

impl State for MatchedState {
  fn get_matched_content(&self) -> &str {
    &self.matched_content.as_str()
  }

  fn is_final_state(&self) -> bool {
    true
  }

  fn transition(self: Box<Self>, c: char) -> Box<dyn State> {
    Box::new(ResetState::new()).consume(c)
  }

  fn consume(self: Box<Self>, c: char) -> Box<dyn State> {
    self.transition(c)
  }
}

// ======================================================================================

pub struct Parser {
  parsed_input: Vec<String>,
  state: Option<Box<dyn State>>,
}

impl Parser {
  pub fn new() -> Self {
    Self {
      parsed_input: Vec::new(),
      state: Some(Box::new(ResetState::new())),
    }
  }

  pub fn get_parsed_input(&self) -> &Vec<String> {
    &self.parsed_input
  }

  pub fn consume(&mut self, c: char) {
    if let Some(s) = self.state.take() {
      let next_state = s.consume(c);

      if next_state.is_final_state() {
        let matched_content = next_state.get_matched_content();
        self.parsed_input.push(matched_content.to_string());
      }

      self.state = Some(next_state);
    }
  }
}
