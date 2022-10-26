mod ascii_distros;
use ascii_distros::ASCII_DISTROS;

use std::env;
use sysinfo::{CpuExt, System, SystemExt};

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
    let os_name_version = format!("{} {}", os_name, sys.os_version().unwrap());
    let kernel = sys.kernel_version().unwrap();
    let desktop = match env::var("XDG_CURRENT_DESKTOP") {
        Ok(val) => val,
        Err(_) => whoami::desktop_env().to_string(),
    };

    // Get hardware info
    let cpu = sys.cpus()[0].brand().to_string();
    let memory_in_mb = format!("{}MiB/{}MiB", sys.used_memory() / 1024 / 1024, sys.total_memory() / 1024 / 1024);

    // Collect everything to a vector
    let printinfo = vec![
        user_at_hostname,
        separator,
        os_name_version,
        kernel,
        desktop,
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
