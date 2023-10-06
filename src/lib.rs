use colored::Colorize;
use regex::Regex;

pub mod print;
pub mod system;

/// Handle inputed line
///
/// # Examples
///
/// ```
/// use demo_console_grep::handle_input;
/// handle_input("grep src/lib.rs");
/// ```
pub fn handle_input(line: &str) {
    let items: Vec<&str> = line.split_whitespace().collect();
    if items.is_empty() {
        print::print_error("no input");
    } else if items[0] != "grep" {
        print::print_error("command not found")
    } else {
        handle_grep_command(items);
    }
}

fn search_no_argument(pattern: &str, path: &str) {
    let contents = match std::fs::read_to_string(path) {
        Ok(rs) => rs,
        Err(e) => {
            print::print_error(e.to_string().as_str());
            return;
        }
    };
    let is_regex = pattern.starts_with('/') && pattern.ends_with('/');
    let regex = if is_regex {
        let pattern = &pattern[1..pattern.len() - 1];
        Regex::new(pattern).unwrap()
    } else {
        Regex::new(&regex::escape(pattern)).unwrap()
    };
    contents.lines().for_each(|line| {
        if let Some(matched) = regex.find(line) {
            let matched_str = matched.as_str();
            let colored_line = line.replace(matched_str, &matched_str.yellow().to_string());
            println!("{line}", line = colored_line);
        }
    });
}

fn search_with_arguments(items: Vec<&str>) {
    let valid_args = [
        "-v",
        "--invert-match",
        "-n",
        "--line-number",
        "-c",
        "--count",
    ];
    let args = &items[1..(items.len() - 2)];
    let pattern = items[items.len() - 2];
    let path = items[items.len() - 1];
    if let Some((idx, _)) = args.iter().enumerate().find(|(_, arg)| {
        !arg.starts_with('-') || arg.chars().all(|c| c == '-') || !valid_args.contains(arg)
    }) {
        print::print_error(format!("invalid option: {}", args[idx]).as_str());
    } else {
        let contents = match std::fs::read_to_string(path) {
            Ok(rs) => rs,
            Err(e) => {
                print::print_error(e.to_string().as_str());
                return;
            }
        };
        let is_regex = pattern.starts_with('/') && pattern.ends_with('/');
        let regex = if is_regex {
            let pattern = &pattern[1..pattern.len() - 1];
            Regex::new(pattern).unwrap()
        } else {
            Regex::new(&regex::escape(pattern)).unwrap()
        };
        let invert_match = args.contains(&"-v") || args.contains(&"--invert-match");
        let print_line_number = args.contains(&"-n") || args.contains(&"--line-number");
        let print_count = args.contains(&"-c") || args.contains(&"--count");
        let mut cnt_pos = 0;
        let mut cnt_neg = 0;
        contents.lines().enumerate().for_each(|(idx, line)| {
            if let Some(matched) = regex.find(line) {
                if !invert_match {
                    let matched_str = matched.as_str();
                    let colored_line = line.replace(matched_str, &matched_str.yellow().to_string());
                    println!(
                        "{}",
                        if print_line_number {
                            format!("{idx}: {line}", idx = idx + 1, line = colored_line)
                        } else {
                            colored_line.to_string()
                        }
                    );
                    cnt_pos += 1;
                }
            } else if invert_match {
                println!(
                    "{}",
                    if print_line_number {
                        format!("{idx}: {line}", idx = idx + 1, line = line)
                    } else {
                        line.to_string()
                    }
                );
                cnt_neg += 1;
            }
        });
        if print_count {
            println!(
                "# of matched lines: {cnt}",
                cnt = (if invert_match { cnt_neg } else { cnt_pos })
                    .to_string()
                    .blue()
            );
        }
    }
}

fn handle_grep_command(items: Vec<&str>) {
    match items.len() {
        1 => {
            print::print_grep();
        }
        2 if items[1] == "--help" => {
            print::print_help();
        }
        2 => {
            print::print_error("not enough input");
        }
        3 if items[1].starts_with('-') => {
            print::print_error("not enough input");
        }
        3 => search_no_argument(items[1], items[2]),
        _ => search_with_arguments(items),
    }
}
