mod ascii_distros;
use ascii_distros::ASCII_DISTROS;

use std::env;
use std::process::Command;
use sysinfo::{CpuExt, System, SystemExt};
use regex::Regex;

fn main() {
    let sys = System::new_all();

    // Get user info
    let username = whoami::username();
    let hostname = whoami::hostname();
    let user_at_hostname = format!("{}@{}", username, hostname);

    // Generate a separator
    let mut separator = String::new();
    for _ in user_at_hostname.chars() {
        separator.push_str("-")
    }

    // Get system info
    let os_name = sys.name().unwrap();
    let os_name = match os_name.as_str() {
        "Darwin" => "MacOS",
        other => other,
    };
    let os_version = match sys.os_version() {
        Some(val) => val,
        None => String::new(),
    };
    let os_name_version = format!("{} {}", os_name, os_version);
    let kernel = sys.kernel_version().unwrap();
    let desktop = match env::var("XDG_CURRENT_DESKTOP") {
        Ok(val) => val,
        Err(_) => whoami::desktop_env().to_string(),
    };

    // Get desktop version
    let desktop_version_cmd = match desktop.as_str() {
        "GNOME" => Some("gnome-shell"),
        "KDE" => Some("plasmashell"),
        "XFCE" => Some("xfce4-session"),
        _ => None,
    };

    let mut desktop_version = String::new();
    if let Some(cmd) = desktop_version_cmd {
        let desktop_version_raw = String::from_utf8(
            Command::new(cmd)
                .arg("--version")
                .output()
                .expect("Couldn't process desktop version")
                .stdout
        ).expect("Couldn't process desktop version");

        let de_ver_regex = Regex::new(r"[0-9]+\.?[0-9]+").unwrap();
        if let Some(val) = de_ver_regex.find(&desktop_version_raw) {
            desktop_version = val.as_str().to_string();
        }
    }


    // Get hardware info
    let cpu = sys.cpus()[0].brand().to_string()
        .replace("Intel(R) Core(TM) ", "")
        .replace("CPU ", "");
    let memory_in_mb = format!("{} MiB / {} MiB", sys.used_memory() / 1024 / 1024, sys.total_memory() / 1024 / 1024);

    // Collect everything to a vector
    let printinfo = vec![
        user_at_hostname,
        separator,
        os_name_version,
        kernel,
        format!("{} {}", desktop, desktop_version),
        cpu,
        memory_in_mb,
    ];

    // Ascii art
    let mut distro_logo = sys.distribution_id();
    if !ASCII_DISTROS.contains_key(&distro_logo) {
        distro_logo = "linux".to_string();
    }
    let ascii_arr: Vec<&str> = ASCII_DISTROS[&distro_logo].split("\n").collect();

    // Print everything
    for (i, value) in (&printinfo).into_iter().enumerate() {
        println!("{} {}", ascii_arr[i], value);
    }

    // Print the rest of the ASCII
    for line in (&printinfo).len()..ascii_arr.len() - 1 {
        println!("{}", ascii_arr[line]);
    }
}
