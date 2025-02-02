use crate::Distro;
use std::{
    io::Result as IOResult,
    process::{
        self,
        Child,
    },
};

pub struct PackageManager {
    distro: Distro,
}

impl PackageManager {
    pub fn try_from_env() -> Option<Self> {
        let distro = Distro::try_from_env()?;

        Some(Self { distro })
    }
    pub fn installed_packages(&self) -> IOResult<Vec<String>> {
        let (mut cmd, args) = self.distro.list_installed_cmd();

        let output_raw = cmd.args(args).output()?;
        let output = String::from_utf8_lossy(&output_raw.stdout);

        let mut output_lines: Vec<String> = output.lines().map(String::from).collect();
        output_lines.sort_unstable();
        output_lines.dedup();

        if let Distro::Debian = self.distro {
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
    }
    pub fn available_packages(&self) -> IOResult<Vec<String>> {
        let (mut cmd, args) = self.distro.list_available_cmd();

        let output_raw = cmd.args(args).output()?;
        let output = String::from_utf8_lossy(&output_raw.stdout);

        let mut output_lines: Vec<String> = output.lines().map(String::from).collect();
        output_lines.sort_unstable();
        output_lines.dedup();

        Ok(output_lines)
    }
    pub fn install(&self, packages: &[String]) -> ! {
        let (mut cmd, args) = self.distro.install_cmd();

        let child: Child = cmd.args(args).args(packages).spawn().unwrap();
        child.wait_with_output().unwrap();
        process::exit(0);
    }
    pub fn remove(&self, packages: &[String]) -> ! {
        let (mut cmd, args) = self.distro.remove_cmd();

        let child: Child = cmd.args(args).args(packages).spawn().unwrap();
        child.wait_with_output().unwrap();
        process::exit(0);
    }
    pub fn query_cmd(&self) -> Option<String> {
        let (cmd, args) = self.distro.query_cmd();

        Some(format!(
            "{} {}",
            cmd.get_program().to_str()?,
            args.join(" ")
        ))
    }
}
