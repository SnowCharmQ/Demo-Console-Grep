use colored::Colorize;

/// Print basic info
///
/// # Examples
///
/// ```
/// use demo_console_grep::print;
/// print::print_info("Hello, world!");
/// ```
pub fn print_info(str: &str) {
    if str.len() < 20 {
        let padding = 20 - str.len();
        let padding_str = " ".repeat(padding / 2);
        let mut info = format!("{}{}{}", padding_str, str, padding_str);
        if padding % 2 != 0 {
            info = format!("{} ", info);
        }
        println!("========= {:<20} =========", info);
    } else {
        let str = format!("{:^40}", str);
        println!("{str}");
    }
}

/// Print grep info
/// 
/// # Examples
/// 
/// ```
/// use demo_console_grep::print;
/// print::print_grep();
/// ```
pub fn print_grep() {
    println!("Usage: grep [OPTION]... PATTERNS [FILE]...");
    println!("Try 'grep --help' for more information.");
}

/// Print help info
/// 
/// # Examples
/// 
/// ```
/// use demo_console_grep::print;
/// print::print_help();
/// ```
pub fn print_help() {
    println!("Usage: grep [OPTION]... PATTERNS [FILE]...");
    println!("Search for PATTERNS in each FILE.");
    println!("Example: grep -c -n /[0-9]/ src/main.rs");
    println!();
    println!("Pattern selection and interpretation:");
    println!("  -v --invert-match     select non-matching lines");
    println!("  -n --line-number      print line number with output lines");
    println!("  -c --count            print a count of matching lines in a file");
}

/// Print error info
/// 
/// # Examples
/// 
/// ```
/// use demo_console_grep::print;
/// print::print_error("Hello, world!");
/// ```
pub fn print_error(str: &str) {
    println!("{}: {}", "error".red(), str);
}
