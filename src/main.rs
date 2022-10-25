use sysinfo::{CpuExt, System, SystemExt};

const ASCII_ART: &str = include_str!("../ascii_distros/fedora.txt");

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
    let desktop = whoami::desktop_env().to_string();

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

    // Print everything
    let ascii_arr: Vec<&str> = ASCII_ART.split("\n").collect();
    for (i, value) in (&printinfo).into_iter().enumerate() {
        println!("{} {}", ascii_arr[i], value);
    }

    // Print the rest of the ASCII
    for line in (&printinfo).len()..ascii_arr.len() {
        println!("{}", ascii_arr[line]);
    }
}
