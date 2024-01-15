use core::fmt;
use std::{collections::HashMap, fs::read_to_string};

use regex::Regex;

use crate::{config::VERSION, function};

#[derive(PartialEq, Eq, Clone)]
pub enum ArgType {
    String(String),
    Number(i32),
}

pub struct FunSweet {
    pub path: String,
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
    pub fn new(path: &String) -> FunSweet {
        let regex_directory_path = Regex::new(r".+/").unwrap();
        let directory_path = regex_directory_path.captures(&path).unwrap()[0].to_string();

        let content = &read_to_string(path).expect("to be a file");
        let mut global_variables = HashMap::new();

        global_variables.insert(
            String::from("version"),
            ArgType::String(VERSION.to_string()),
        );

        return FunSweet {
            path: directory_path,
            taking_string: false,
            taking_number: false,
            taking_variable: false,
            function_active: false,
            function_name: String::from(""),
            function_args: Vec::new(),
            stored: global_variables,
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

pub fn pass_arg(funsweet: &mut FunSweet) {
    let k = &funsweet.word.trim().to_string();

    if k.starts_with("!") {
        let sliced_key = k[1..k.len()].to_string();
        let v = funsweet.stored.get(&sliced_key);

        match v {
            Some(v) => {
                funsweet.function_args.push(v.clone());
            }
            _ => {}
        }
    } else if k.starts_with("\"") & k.ends_with("\"") {
        let sliced_string = k[1..k.len() - 1].to_string();
        funsweet.function_args.push(ArgType::String(sliced_string));
    } else if k.chars().all(|x| x.is_numeric()) {
        let numeric = k.parse().unwrap();
        funsweet.function_args.push(ArgType::Number(numeric));
    }
}

fn parse_arg(funsweet: &mut FunSweet) {
    while funsweet.char != ";" && funsweet.char != "," {
        funsweet.accept();
        funsweet.next();
    }

    pass_arg(funsweet);
}

fn parse_function(funsweet: &mut FunSweet) {
    while funsweet.char != ";" {
        while funsweet.char.trim().is_empty() == false {
            funsweet.next();
        }

        parse_arg(funsweet);
        funsweet.reset();
    }

    if funsweet.function_name == "Print" || funsweet.function_name == "Warn" {
        function::output(funsweet);
    } else if funsweet.function_name == "Store" {
        function::store(funsweet);
    } else if funsweet.function_name == "Drop" {
        function::drop(funsweet);
    } else if funsweet.function_name == "Run" {
        function::run(funsweet);
    } else if funsweet.function_name == "Operation" {
        function::math_operation(funsweet);
    }
}

fn parse_comment(funsweet: &mut FunSweet) {
    while funsweet.char != "\n" {
        funsweet.next();
    }

    funsweet.reset();
}

pub fn parse_content(content: String) -> FunSweet {
    let mut funsweet = FunSweet::new(&content);

    while funsweet.index < funsweet.content.len() {
        funsweet.next();

        if funsweet.word == "//" {
            parse_comment(&mut funsweet);
        } else if funsweet.char == ";" {
            funsweet.reset();
        } else if funsweet.char == "<" {
            funsweet.function_name = funsweet.word.clone();
            funsweet.function_args.clear();
            funsweet.reset();

            parse_function(&mut funsweet);
        } else if funsweet.char.trim().is_empty() {
            // do nothing
        } else {
            funsweet.accept();
        }
    }

    return funsweet;
}
