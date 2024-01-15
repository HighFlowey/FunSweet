use crate::{
    config::OUTPUT,
    funsweet::{self, ArgType, FunSweet},
};
use inline_colorization::*;

fn variant_eq<T>(a: &T, b: &T) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

pub fn run(funsweet: &mut FunSweet) {
    if funsweet.function_args.len() < 1 {
        return;
    }

    let arg0 = &funsweet.function_args[0];

    match arg0 {
        ArgType::Number(_) => return,
        ArgType::String(path) => {
            let relative_path = funsweet.path.clone() + path;
            let module = funsweet::parse_content(relative_path);
            funsweet.stored.extend(module.stored);
        }
    }
}

pub fn store(funsweet: &mut FunSweet) {
    if funsweet.function_args.len() < 2 {
        return;
    }

    let k = &funsweet.function_args[0];
    let v = &funsweet.function_args[1];

    match k {
        ArgType::Number(_) => return,
        ArgType::String(k) => {
            for char in k.chars().into_iter() {
                if char.is_whitespace() {
                    println!("err: key can't contain whitespace");
                    return;
                }
                // else if char.is_numeric() {
                //     println!("err: key can't contain numbers");
                //     return;
                // }
            }
            funsweet.stored.insert(k.to_string(), v.clone());
        }
    }
}

pub fn drop(funsweet: &mut FunSweet) {
    if funsweet.function_args.len() < 1 {
        return;
    }

    let k = &funsweet.function_args[0];

    match k {
        ArgType::Number(_) => return,
        ArgType::String(k) => {
            for char in k.chars().into_iter() {
                if char.is_whitespace() {
                    println!("err: key can't contain whitespace");
                    return;
                }
                // else if char.is_numeric() {
                //     println!("err: key can't contain numbers");
                //     return;
                // }
            }
            funsweet.stored.remove_entry(k);
        }
    }
}

pub fn math_operation(funsweet: &mut FunSweet) {
    if funsweet.function_args.len() < 4 {
        return;
    }

    let key = &funsweet.function_args[0];
    let arg1 = &funsweet.function_args[1];
    let operation = &funsweet.function_args[2];
    let arg2 = &funsweet.function_args[3];

    let ArgType::String(key) = key else {
        println!("Err: first argument is used for variable name which should be a string");
        return;
    };

    let ArgType::String(operation) = operation else {
        return;
    };

    if variant_eq(arg1, arg2) != true {
        println!("Err: both arguments should have the same type");
        return;
    }

    match arg1 {
        ArgType::Number(arg1) => {
            let ArgType::Number(arg2) = arg2 else {
                return;
            };

            // Numbers
            if operation == &String::from("+") {
                funsweet
                    .stored
                    .insert(key.clone(), ArgType::Number(arg1 + arg2));
            } else if operation == &String::from("-") {
                funsweet
                    .stored
                    .insert(key.clone(), ArgType::Number(arg1 - arg2));
            } else if operation == &String::from("*") {
                funsweet
                    .stored
                    .insert(key.clone(), ArgType::Number(arg1 * arg2));
            } else if operation == &String::from("/") {
                funsweet
                    .stored
                    .insert(key.clone(), ArgType::Number(arg1 / arg2));
            } else if operation == &String::from("^") {
                funsweet
                    .stored
                    .insert(key.clone(), ArgType::Number(arg1 ^ arg2));
            } else if operation == &String::from("%") {
                funsweet
                    .stored
                    .insert(key.clone(), ArgType::Number(arg1 % arg2));
            }
        }
        ArgType::String(arg1) => {
            let ArgType::String(arg2) = arg2 else {
                return;
            };

            // Strings
            if operation == &String::from("+") {
                funsweet
                    .stored
                    .insert(key.clone(), ArgType::String(arg1.clone() + arg2));
            }
        }
    }
}

pub fn output(funsweet: &FunSweet) {
    if OUTPUT == false {
        return;
    }

    if funsweet.function_args.len() < 1 {
        return;
    }

    let iter = funsweet.function_args.iter();
    let last = funsweet.function_args.iter().last().unwrap();

    let color = if funsweet.function_name == "Print" {
        color_white
    } else {
        color_yellow
    };

    for i in iter {
        let is_last = i == last;

        if is_last == false {
            print!("{color}{}, {color_reset}", i);
        } else {
            print!("{color}{}{color_reset}\n", i);
        }
    }
}
