use demo_console_grep::print;
use demo_console_grep::system;

fn main() {
    print::print_info("Welcome to demo-console-grep!");
    print::print_info(format!("OS: {}", system::get_os()).as_str());
    print::print_info(format!("Version: {}", system::get_version()).as_str());
}
