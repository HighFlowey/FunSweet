use homedir::get_my_home;
use std::{
    env::{current_exe, join_paths},
    fs::{copy, create_dir},
};

pub fn main() {
    let mut home = get_my_home().expect("to exist").expect("to exist");
    home.push(".funsweet");

    let exe_path = current_exe().expect("to be executable");
    let mut exe_dir_path = exe_path.clone();
    exe_dir_path.pop();

    if exe_dir_path == home {
        // already running the installed executable
        return;
    }

    if home.is_dir() == false {
        create_dir(&home).expect("to create directory");
        join_paths([&home]).expect("to join path");
    }

    home.push("funsweet.exe");
    copy(exe_path, &home).expect("to copy file");
}
