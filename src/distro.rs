use crate::{
    DISTRO_BINARY_MAP,
    FS_ROOT,
};
use std::{
    fs,
    process::Command,
};

#[derive(Clone)]
pub enum Distro {
    Arch,
    Debian,
    Void,
    OpenSuse,
}

impl Distro {
    pub fn try_from_env<'a>() -> Option<Self> {
        for (distro, binary_path) in &*DISTRO_BINARY_MAP {
            if fs::exists(binary_path).unwrap_or(false) {
                return Some(distro.clone());
            }
        }

        None
    }
    pub fn list_installed_cmd(&self) -> (Command, Vec<&str>) {
        match self {
            Self::Arch => (Command::new("/usr/bin/pacman"), vec!["-Q", "-q"]),
            Self::Debian => (
                Command::new(format!("{}/usr/bin/dpkg", *FS_ROOT)),
                vec!["--get-selections"],
            ),
            _ => todo!(),
        }
    }
    pub fn list_available_cmd(&self) -> (Command, Vec<&str>) {
        match self {
            Self::Arch => (Command::new("/usr/bin/pacman"), vec!["-S", "-l", "-q"]),
            Self::Debian => (
                Command::new(format!("{}/usr/bin/apt-cache", *FS_ROOT)),
                vec!["pkgnames", "--generate"],
            ),
            _ => todo!(),
        }
    }
    pub fn install_cmd(&self) -> (Command, Vec<&str>) {
        match self {
            Self::Arch => (Command::new("/usr/bin/pacman"), vec!["-S"]),
            Self::Debian => (
                Command::new(format!("{}/usr/bin/apt", *FS_ROOT)),
                vec!["install"],
            ),
            _ => todo!(),
        }
    }
    pub fn remove_cmd(&self) -> (Command, Vec<&str>) {
        match self {
            Self::Arch => (Command::new("/usr/bin/pacman"), vec!["-R", "-n", "-s"]),
            Self::Debian => (
                Command::new(format!("{}/usr/bin/apt", *FS_ROOT)),
                vec!["remove"],
            ),
            _ => todo!(),
        }
    }
    pub fn query_cmd(&self) -> (Command, Vec<&str>) {
        match self {
            Self::Arch => (Command::new("/usr/bin/pacman"), vec!["-S", "-i"]),
            Self::Debian => (
                Command::new(format!("{}/usr/bin/apt-cache", *FS_ROOT)),
                vec!["show"],
            ),
            _ => todo!(),
        }
    }
}
