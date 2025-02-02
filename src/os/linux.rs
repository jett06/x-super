use crate::{
    PackageManagerImpl,
    OS_RELEASE_PATH,
};
use ini::Ini;
use std::{
    fs,
    io::Result as IOResult,
    path::PathBuf,
    process::{
        self,
        Command,
    },
};

#[derive(Clone)]
pub enum LinuxDistro {
    Arch,
    Debian,
    Void,
}

impl LinuxDistro {
    fn run_program_and_return_deduped_lines(
        program_path: PathBuf, args: &[&str],
    ) -> IOResult<Vec<String>> {
        let mut cmd = Command::new(program_path);

        let output_raw = cmd.args(args).output()?;
        let output = String::from_utf8_lossy(&output_raw.stdout);

        let mut output_lines: Vec<String> = output.lines().map(String::from).collect();
        output_lines.sort_unstable();
        output_lines.dedup();

        Ok(output_lines)
    }
    fn run_program_interactive(program_path: PathBuf, args: &[&str]) -> IOResult<()> {
        let mut cmd = Command::new(program_path);
        let child = cmd.args(args).spawn()?;
        child.wait_with_output()?;

        Ok(())
    }
    fn from_os_release(os_release: &Ini) -> Option<Self> {
        let section = os_release.general_section();
        let id = section.get("ID");

        match id {
            Some("debian") | Some("pureos") | Some("Deepin") | Some("linuxmint") => {
                Some(Self::Debian)
            }
            Some("arch") | Some("manjaro-arm") | Some("garuda") | Some("artix") => Some(Self::Arch),
            Some("void") => Some(Self::Void),
            _ => None,
        }
    }
    pub fn try_from_env() -> Option<Self> {
        if fs::exists(OS_RELEASE_PATH).unwrap_or(false) {
            let os_release = Ini::load_from_file(OS_RELEASE_PATH).ok()?;

            Self::from_os_release(&os_release)
        } else {
            None
        }
    }
}

impl PackageManagerImpl for LinuxDistro {
    fn installed_package_list(&self) -> IOResult<Vec<String>> {
        let (program_name, args) = match self {
            Self::Arch => ("pacman", vec!["-Q", "-q"]),
            Self::Debian => ("dpkg", vec!["--get-selections"]),
            _ => todo!(),
        };

        if let Ok(program_path) = which::which(program_name) {
            let output_lines = Self::run_program_and_return_deduped_lines(program_path, &args)?;

            if let Self::Debian = self {
                let mut package_list = Vec::new();

                for line in output_lines {
                    if let Some(first_word) = line.split_whitespace().next() {
                        package_list.push(first_word.to_string());
                    }
                }

                Ok(package_list)
            } else {
                Ok(output_lines)
            }
        } else {
            todo!("implement err handling when necessary pkg manager isn't found by `which::which`")
        }
    }
    fn available_package_list(&self) -> IOResult<Vec<String>> {
        let (program_name, args) = match self {
            Self::Arch => ("pacman", vec!["-S", "-l", "-q"]),
            Self::Debian => ("apt-cache", vec!["pkgnames", "--generate"]),
            _ => todo!(),
        };

        if let Ok(program_path) = which::which(program_name) {
            let output_lines = Self::run_program_and_return_deduped_lines(program_path, &args)?;

            Ok(output_lines)
        } else {
            todo!()
        }
    }
    fn interactive_install(&self, packages: &[String]) -> ! {
        let (program_name, args) = match self {
            Self::Arch => ("pacman", vec!["-S"]),
            Self::Debian => ("apt", vec!["install"]),
            _ => todo!(),
        };

        if let Ok(program_path) = which::which(program_name) {
            let mut full_args = args.clone();
            full_args.extend(packages.iter().map(String::as_str));

            if let Err(e) = Self::run_program_interactive(program_path, &full_args) {
                eprintln!(
                    "Running the package manager program interactively failed! Error: {:#?}",
                    e
                );
                process::exit(1);
            }

            process::exit(0);
        } else {
            eprintln!("Failed to locate the package manager binary!");
            process::exit(1);
        }
    }
    fn interactive_remove(&self, packages: &[String]) -> ! {
        let (program_name, args) = match self {
            Self::Arch => ("pacman", vec!["-R", "-n", "-s"]),
            Self::Debian => ("apt", vec!["remove"]),
            _ => todo!(),
        };

        if let Ok(program_path) = which::which(program_name) {
            let mut full_args = args.clone();
            full_args.extend(packages.iter().map(String::as_str));

            if let Err(e) = Self::run_program_interactive(program_path, &full_args) {
                eprintln!(
                    "Running the package manager program interactively failed! Error: {:#?}",
                    e
                );
                process::exit(1);
            }

            process::exit(0);
        } else {
            eprintln!("Failed to locate the package manager binary!");
            process::exit(1);
        }
    }
    fn package_query_cmd(&self) -> String {
        let (program_name, args) = match self {
            Self::Arch => ("pacman", vec!["-S", "-i"]),
            Self::Debian => ("apt-cache", vec!["show"]),
            _ => todo!(),
        };

        if let Ok(program_path) = which::which(program_name) {
            format!("{} {}", program_path.display(), args.join(" "))
        } else {
            todo!()
        }
    }
}
