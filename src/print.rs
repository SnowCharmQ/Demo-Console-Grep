/// Print info
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
