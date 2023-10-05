use std::io::{self, BufRead};

use colored::Colorize;
use demo_console_grep::handle_input;
use demo_console_grep::print;
use demo_console_grep::system;

fn main() {
    let cut_off = "=".repeat(40);
    println!("{}", cut_off.red());
    print::print_info("Welcome to demo-console-grep!");
    print::print_info(format!("OS: {}", system::get_os()).as_str());
    print::print_info(format!("Version: {}", system::get_version()).as_str());
    main_loop();
    println!("{}", cut_off.red());
}

fn main_loop() {
    let cut_off = "=".repeat(40);
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    loop {
        let line = lines.next().unwrap().unwrap();
        if line.eq_ignore_ascii_case("exit") || line.eq_ignore_ascii_case("quit") {
            break;
        } else {
            handle_input(&line);
        }
        println!("{}", cut_off.green());
    }
}
