use crate::DISTRO_BINARY_MAP;
use std::fs;

#[derive(Clone)]
pub enum Distro {
    Arch,
    Debian,
    Void,
    OpenSuse,
}

impl Distro {
    pub fn try_from_env<'a>() -> Option<(Self, &'a str)> {
        for (distro, binary_path) in &DISTRO_BINARY_MAP {
            if fs::exists(binary_path).unwrap_or(false) {
                return Some((distro.clone(), binary_path));
            }
        }

        None
    }
}
