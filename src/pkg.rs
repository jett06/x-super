use crate::Distro;
use std::process::{
    self,
    Child,
    Command,
};

pub struct PackageManager {
    distro: Distro,
    binary: String,
}

impl PackageManager {
    pub fn init() -> Self {
        let (distro, binary_ref) = Distro::try_from_env().unwrap();

        Self {
            distro,
            binary: binary_ref.to_string(),
        }
    }
    fn output_adapter_with_dedup(output: &[u8]) -> Vec<String> {
        let mut output_lines: Vec<String> = String::from_utf8_lossy(output)
            .lines()
            .into_iter()
            .map(String::from)
            .collect();
        output_lines.sort_unstable();
        output_lines.dedup();

        output_lines
    }
    pub fn installed_packages(&self) -> Vec<String> {
        let mut cmd = Command::new(self.binary.clone());

        match &self.distro {
            Distro::Arch => cmd.arg("-Q").arg("-q"),
            _ => todo!("PackageManager::installed_packages#[`match self.distro`]"),
        };

        Self::output_adapter_with_dedup(&cmd.output().unwrap().stdout)
    }
    pub fn available_packages(&self) -> Vec<String> {
        let mut cmd = Command::new(self.binary.clone());

        match &self.distro {
            Distro::Arch => cmd.arg("-S").arg("-l").arg("-q"),
            _ => todo!("PackageManager::available_packages#[`match self.distro`]"),
        };

        Self::output_adapter_with_dedup(&cmd.output().unwrap().stdout)
    }
    pub fn install(&self, packages: String) -> ! {
        let mut cmd = Command::new(self.binary.clone());

        match &self.distro {
            Distro::Arch => cmd.arg("-S").arg(packages),
            _ => todo!("PackageManager::install#[`match self.distro`]"),
        };

        let child: Child = cmd.spawn().unwrap();
        child.wait_with_output().unwrap();
        process::exit(0);
    }
    pub fn remove(&self, packages: String) -> ! {
        let mut cmd = Command::new(self.binary.clone());

        match &self.distro {
            Distro::Arch => cmd.arg("-R").arg("-n").arg("-s").arg(packages),
            _ => todo!("PackageManager::remove#[`match self.distro`]"),
        };

        let child: Child = cmd.spawn().unwrap();
        child.wait_with_output().unwrap();
        process::exit(0);
    }
    pub fn query_cmd(&self) -> String {
        match self.distro {
            Distro::Arch => format!("{} -Si", self.binary.clone()),
            _ => todo!("PackageManager::info_cmd#[`match self.distro`]"),
        }
    }
}
