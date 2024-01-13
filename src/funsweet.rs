use core::fmt;
use std::collections::HashMap;

use crate::function;

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

pub fn pass_arg(mut funsweet: FunSweet) -> FunSweet {
    let k = funsweet.word.clone();
    let v = funsweet.stored.get(&k);
    match v {
        Some(v) => {
            funsweet.function_args.push(v.clone());
        }
        _ => {}
    }
    return funsweet;
}

pub fn parse_content(content: String) {
    let mut funsweet = FunSweet::new(&content);

    while funsweet.index < funsweet.content.len() {
        funsweet.next();

        if funsweet.word == "//" {
            while funsweet.char != "\n" {
                funsweet.next();
            }

            funsweet.reset();
        }

        if funsweet.taking_string == true && funsweet.char != "\"" {
            funsweet.accept();
        } else if funsweet.char == ";" {
            if funsweet.taking_number == true {
                funsweet.taking_number = false;
                funsweet
                    .function_args
                    .push(ArgType::Number(funsweet.word.parse().unwrap()))
            }

            if funsweet.taking_variable == true {
                funsweet.taking_variable = false;
                funsweet = pass_arg(funsweet);
            }

            if funsweet.function_active == true {
                funsweet.function_active = false;

                if funsweet.function_name == "Print" || funsweet.function_name == "Warn" {
                    function::output(&funsweet);
                } else if funsweet.function_name == "Store" {
                    function::store(&mut funsweet);
                } else if funsweet.function_name == "Drop" {
                    function::drop(&mut funsweet);
                }
            }

            funsweet.function_name = String::from("");
            funsweet.function_args.clear();
            funsweet.reset();
        } else if funsweet.char == "<" {
            let function_name = funsweet.word.trim().to_string();
            funsweet.function_name = function_name.clone();
            funsweet.reset();
            funsweet.function_active = true;
        } else if funsweet.function_active == true {
            if funsweet.char == "\"" {
                if funsweet.taking_string == true {
                    funsweet
                        .function_args
                        .push(ArgType::String(funsweet.word.clone()));
                    funsweet.taking_string = false;
                    funsweet.reset();
                } else {
                    funsweet.taking_string = true;
                    funsweet.reset();
                }
            } else if funsweet.char.chars().next().unwrap().is_numeric() {
                if funsweet.taking_number == false && funsweet.taking_variable == false {
                    funsweet.taking_number = true;
                    funsweet.reset();
                }

                funsweet.accept();
            } else if funsweet.char == "," {
                if funsweet.taking_number == true {
                    funsweet.taking_number = false;
                    funsweet
                        .function_args
                        .push(ArgType::Number(funsweet.word.parse().unwrap()))
                } else if funsweet.taking_variable == true {
                    funsweet.taking_variable = false;
                    funsweet = pass_arg(funsweet);
                } else if funsweet.word.is_empty() == false {
                    funsweet
                        .function_args
                        .push(ArgType::String(funsweet.word.clone()));
                }

                funsweet.reset()
            } else if funsweet.char == "!" {
                // looking for variable
                funsweet.taking_variable = true;
            } else if funsweet.taking_variable && funsweet.char.trim().is_empty() == false {
                funsweet.accept();
            }
        } else if funsweet.char.trim().is_empty() {
            // ignore
        } else {
            funsweet.accept();
        }
    }
}
