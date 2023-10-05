enum SystemType {
    _MacOS,
    _LinuxOS,
    _WinOS,
    _UnknownOS,
}

#[cfg(target_os = "_MacOS")]
const SYS_TYPE: SystemType = SystemType::_MacOS;
#[cfg(target_os = "linux")]
const SYS_TYPE: SystemType = SystemType::_LinuxOS;
#[cfg(target_os = "windows")]
const SYS_TYPE: SystemType = SystemType::_WinOS;
#[cfg(not(any(target_os = "_MacOS", target_os = "linux", target_os = "windows")))]
const SYS_TYPE: SystemType = SystemType::_UnknownOS;

/// Get OS info
///
/// # Examples
///
/// ```
/// use demo_console_grep::system;
/// let os = system::get_os();
/// assert_ne!(os, "Unknown");
/// ```
pub fn get_os() -> String {
    match SYS_TYPE {
        SystemType::_MacOS => String::from("MacOS"),
        SystemType::_LinuxOS => String::from("Linux"),
        SystemType::_WinOS => String::from("Windows"),
        SystemType::_UnknownOS => String::from("Unknown"),
    }
}

/// Get version info
///
/// # Examples
///
/// ```
/// use demo_console_grep::system;
/// let version = system::get_version();
/// println!("{}", version);
/// ```
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
