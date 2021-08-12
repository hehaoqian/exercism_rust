use std::collections::HashMap;

pub type Value = i32;
pub type ForthResult = Result<(), Error>;

#[derive(Default)]
pub struct Forth {
    stack: Vec<Value>,
    def: HashMap<String, Vec<String>>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn stack(&self) -> &[Value] {
        self.stack.as_slice()
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }
    fn push_twice(&mut self, v0: Value, v1: Value) {
        self.stack.push(v0);
        self.stack.push(v1);
    }

    fn pop(&mut self) -> Result<Value, Error> {
        if let Some(v) = self.stack.pop() {
            Ok(v)
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn pop_twice(&mut self) -> Result<(Value, Value), Error> {
        let first = self.pop()?;
        let second = self.pop()?;
        Ok((first, second))
    }

    fn process(&mut self, tokens: &[&str]) -> ForthResult {
        let mut def_content = Vec::<String>::new();
        let mut defining = false;
        for &token in tokens.iter() {
            let token = token.to_ascii_lowercase();
            let token = token.as_str();
            if defining {
                match token {
                    ":" => return Err(Error::InvalidWord),
                    ";" => {
                        if def_content.is_empty() {
                            return Err(Error::InvalidWord);
                        } else {
                            let key = def_content.remove(0);
                            self.def.insert(key, def_content.clone());
                            def_content.clear();
                        }
                    }
                    _ => def_content.push(token.to_owned()),
                }
            } else if let Ok(num) = token.parse::<Value>() {
                self.push(num);
            } else {
                match token {
                    ":" => {
                        defining = true;
                    }
                    _ if self.def.contains_key(token) => {
                        let tokens;
                        {
                            let def = self.def.get(token).unwrap();
                            tokens = def.iter().map(|s| s.clone()).collect::<Vec<_>>();
                        }
                        self.process(
                            tokens
                                .iter()
                                .map(|t| t.as_str())
                                .collect::<Vec<_>>()
                                .as_slice(),
                        )?;
                    }
                    "+" => {
                        let numbers = self.pop_twice()?;
                        self.push(numbers.1 + numbers.0);
                    }
                    "-" => {
                        let numbers = self.pop_twice()?;
                        self.push(numbers.1 - numbers.0);
                    }
                    "*" => {
                        let numbers = self.pop_twice()?;
                        self.push(numbers.1 * numbers.0);
                    }
                    "/" => {
                        let numbers = self.pop_twice()?;
                        if numbers.0 == 0 {
                            return Err(Error::DivisionByZero);
                        } else {
                            self.push(numbers.1 / numbers.0);
                        }
                    }
                    "dup" => {
                        let num = self.pop()?;
                        self.push_twice(num, num);
                    }
                    "drop" => {
                        self.pop()?;
                    }
                    "swap" => {
                        let numbers = self.pop_twice()?;
                        self.push_twice(numbers.0, numbers.1);
                    }
                    "over" => {
                        let numbers = self.pop_twice()?;
                        self.push_twice(numbers.1, numbers.0);
                        self.push(numbers.1);
                    }
                    _ => {
                        return Err(Error::UnknownWord);
                    }
                }
            }
        }
        Ok(())
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        let tokens = input.split_whitespace().collect::<Vec<_>>();
        self.process(tokens.as_slice())
    }
}
