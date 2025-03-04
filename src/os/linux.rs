use crate::{
    err::{
        Error,
        Result,
    },
    pkg::PackageManagerImpl,
    sudo::ElevationHandler,
};
use std::process::Command;
#[cfg(not(target_os = "android"))]
use {
    crate::OS_RELEASE_PATH,
    ini::Ini,
    std::fs,
};

#[derive(Clone)]
pub enum LinuxDistro {
    Arch,
    Debian,
    Void,
}

impl LinuxDistro {
    fn run_cmd_and_return_deduped_lines(mut cmd: Command) -> Result<Vec<String>> {
        let output_raw = cmd.output()?;
        let output = String::from_utf8(output_raw.stdout)?;

        let mut output_lines: Vec<String> = output.lines().map(String::from).collect();
        output_lines.sort_unstable();
        output_lines.dedup();

        Ok(output_lines)
    }
    fn run_cmd_interactive(mut cmd: Command) -> Result<()> {
        let child = cmd.spawn()?;
        child.wait_with_output()?;

        Ok(())
    }
    #[cfg(not(target_os = "android"))]
    fn from_os_release(os_release: &Ini) -> Result<Self> {
        let section = os_release.general_section();
        let id = section.get("ID");

        match id {
            Some("debian") | Some("pureos") | Some("Deepin") | Some("linuxmint") => {
                Ok(Self::Debian)
            }
            Some("arch") | Some("manjaro-arm") | Some("garuda") | Some("artix") => Ok(Self::Arch),
            Some("void") => Ok(Self::Void),
            _ => Err(Error::UnsupportedOS),
        }
    }
    #[cfg(not(target_os = "android"))]
    pub fn try_from_env() -> Result<Self> {
        if fs::exists(OS_RELEASE_PATH).unwrap_or(false) {
            let os_release = Ini::load_from_file(OS_RELEASE_PATH)?;

            Self::from_os_release(&os_release)
        } else {
            Err(Error::UnsupportedOS)
        }
    }
    // Termux doesn't have the `/etc/os-release` file, so we just create a stub function since its
    // `apt` emulates Debian's.
    #[cfg(target_os = "android")]
    pub fn try_from_env() -> Result<Self> {
        Ok(Self::Debian)
    }
}

impl PackageManagerImpl for LinuxDistro {
    fn installed_package_list(&self) -> Result<Vec<String>> {
        let (program_name, args) = match self {
            Self::Arch => ("pacman", vec!["-Q", "-q"]),
            Self::Debian => ("dpkg", vec!["--get-selections"]),
            Self::Void => ("xbps-query", vec!["--search="]),
        };

        let program_path = which::which(program_name)?;
        let mut cmd = Command::new(program_path);
        cmd.args(args);

        let output_lines = Self::run_cmd_and_return_deduped_lines(cmd)?;

        match self {
            Self::Debian => {
                let mut package_list = Vec::new();

                for line in output_lines {
                    if let Some(first_word) = line.split_whitespace().next() {
                        package_list.push(first_word.to_string());
                    }
                }

                Ok(package_list)
            }
            Self::Void => {
                let mut package_list = Vec::new();

                for line in output_lines {
                    if let Some(second_word) = line.split_whitespace().nth(1) {
                        if let Some(last_hyphen_index) = second_word.rfind("-") {
                            package_list.push(second_word[..last_hyphen_index].to_string())
                        }
                    }
                }

                Ok(package_list)
            }
            _ => Ok(output_lines),
        }
    }
    fn available_package_list(&self) -> Result<Vec<String>> {
        let (program_name, args) = match self {
            Self::Arch => ("pacman", vec!["-S", "-l", "-q"]),
            Self::Debian => ("apt-cache", vec!["pkgnames", "--generate"]),
            Self::Void => ("xbps-query", vec!["-R", "--search="]),
        };

        let program_path = which::which(program_name)?;
        let mut cmd = Command::new(program_path);
        cmd.args(args);

        let output_lines = Self::run_cmd_and_return_deduped_lines(cmd)?;

        if let Self::Void = self {
            let mut package_list = Vec::new();

            for line in output_lines {
                if let Some(second_word) = line.split_whitespace().nth(1) {
                    if let Some(last_hyphen_index) = second_word.rfind("-") {
                        package_list.push(second_word[..last_hyphen_index].to_string())
                    }
                }
            }

            Ok(package_list)
        } else {
            Ok(output_lines)
        }
    }
    fn interactive_install(
        &self, packages: &[String], maybe_elevation_handler: Option<ElevationHandler>,
    ) -> Result<()> {
        let (program_name, mut args) = match self {
            Self::Arch => ("pacman", vec!["-S"]),
            Self::Debian => ("apt", vec!["install"]),
            Self::Void => ("xbps-install", vec!["-S"]),
        };

        let program_path = which::which(program_name)?;
        args.extend(packages.iter().map(String::as_str));
        let mut cmd = Command::new(program_path);
        cmd.args(args);

        if let Some(elevation) = maybe_elevation_handler {
            if elevation.should_elevate() {
                cmd = elevation.elevate_cmd(cmd)?;
            }
        }

        Self::run_cmd_interactive(cmd)
    }
    fn interactive_remove(
        &self, packages: &[String], maybe_elevation_handler: Option<ElevationHandler>,
    ) -> Result<()> {
        let (program_name, mut args) = match self {
            Self::Arch => ("pacman", vec!["-R", "-n", "-s"]),
            Self::Debian => ("apt", vec!["remove"]),
            Self::Void => ("xbps-remove", vec!["-R"]),
        };

        let program_path = which::which(program_name)?;
        args.extend(packages.iter().map(String::as_str));
        let mut cmd = Command::new(program_path);
        cmd.args(args);

        if let Some(elevation) = maybe_elevation_handler {
            if elevation.should_elevate() {
                cmd = elevation.elevate_cmd(cmd)?;
            }
        }

        Self::run_cmd_interactive(cmd)
    }
    fn package_query_cmd(&self) -> Result<String> {
        let (program_name, args) = match self {
            Self::Arch => ("pacman", vec!["-S", "-i"]),
            Self::Debian => ("apt-cache", vec!["show"]),
            Self::Void => ("xbps-query", vec!["-R", "-S"]),
        };

        let program_path = which::which(program_name)?;
        Ok(format!("{} {}", program_path.display(), args.join(" ")))
    }
}
