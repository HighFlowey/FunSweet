use core::fmt;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone)]
pub enum ArgType {
    String(String),
    Number(i32),
}

pub struct FunSweet {
    pub taking_string: bool,
    pub taking_number: bool,
    pub taking_variable: bool,
    pub function_active: bool,
    pub function_name: String,
    pub function_args: Vec<ArgType>,
    pub stored: HashMap<String, ArgType>,
    pub index: usize,
    pub content: String,
    pub char: String,
    pub word: String,
}

impl FunSweet {
    pub fn new(content: &String) -> FunSweet {
        return FunSweet {
            taking_string: false,
            taking_number: false,
            taking_variable: false,
            function_active: false,
            function_name: String::from(""),
            function_args: Vec::new(),
            stored: HashMap::new(),
            index: 0,
            content: content.clone(),
            char: String::from(""),
            word: String::from(""),
        };
    }

    pub fn next(&mut self) {
        self.char = self.content[self.index..self.index + 1].to_string();
        self.index += 1;
    }

    pub fn accept(&mut self) {
        self.word += &self.char.as_str();
    }

    pub fn reset(&mut self) {
        self.word = String::from("");
    }
}

impl fmt::Display for ArgType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArgType::String(string) => {
                write!(f, "{}", string)
            }
            ArgType::Number(number) => {
                write!(f, "{}", number)
            }
        }
    }
}
