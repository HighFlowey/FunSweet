use crate::{
    config::OUTPUT,
    funsweet::{ArgType, FunSweet},
};
use inline_colorization::*;

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
