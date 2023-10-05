use colored::Colorize;

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
    dbg!("{}", line);
    let items: Vec<&str> = line.split_whitespace().collect();
    if items.len() < 1 {
        let info = format!("{}: no input", "error".red());
        println!("{info}");
        return;
    } else if items[0] != "grep" {
        let info = format!("{}: command not found", "error".red());
        println!("{info}");
        return;
    } else {
        handle_grep_command(items);
    }
}

fn search(pattern: &str, path: &str) {
    let contents = match std::fs::read_to_string(path) {
        Ok(rs) => rs,
        Err(e) => {
            println!("{}: {}", "error".red(), e);
            return;
        }
    };
    contents.lines().enumerate().for_each(|(idx, line)| {
        if line.contains(pattern) {
            println!("{idx}: {line}");
        }
    });
}

fn handle_grep_command(items: Vec<&str>) {
    match items.len() {
        1 => {
            println!("Usage: grep [OPTION]... PATTERNS [FILE]...");
            println!("Try 'grep --help' for more information.");
        }
        2 => {
            let info = format!("{}: not enough input", "error".red());
            println!("{}", info);
        }
        3 if items[1].starts_with("-") => {
            let info = format!("{}: not enough input", "error".red());
            println!("{}", info);
        }
        3 => search(items[1], items[2]),
        _ => {}
    }
}
