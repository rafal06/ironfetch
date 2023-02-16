use phf::{Map, phf_map};

pub static ASCII_DISTROS: Map<&'static str, &'static str> = phf_map! {
    "macos"       => include_str!("../ascii_distros/macos.txt"       ),
    "arch"        => include_str!("../ascii_distros/arch.txt"        ),
    "artix"       => include_str!("../ascii_distros/artix.txt"       ),
    "debian"      => include_str!("../ascii_distros/debian.txt"      ),
    "endeavouros" => include_str!("../ascii_distros/endeavouros.txt" ),
    "fedora"      => include_str!("../ascii_distros/fedora.txt"      ),
    "opensuse-tumbleweed"      => include_str!("../ascii_distros/opensuse-tumbleweed.txt"      ),
    "raspbian"    => include_str!("../ascii_distros/raspbian.txt"    ),
    "linux"       => include_str!("../ascii_distros/linux.txt"       ),
};
