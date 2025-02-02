use crate::err::{
    Error,
    Result,
};
use std::{
    fmt::{
        Display,
        Formatter,
        Result as FmtResult,
    },
    process::Command,
};

pub enum ElevationHandler {
    Doas,
    Sudo,
    Gsudo,
    Pkexec,
    Please,
}

impl ElevationHandler {
    pub fn try_from_env() -> Result<Self> {
        which::which("doas")
            .map(|_| Self::Doas)
            .or_else(|_| which::which("sudo").map(|_| Self::Sudo))
            .or_else(|_| which::which("gsudo").map(|_| Self::Gsudo))
            .or_else(|_| which::which("pkexec").map(|_| Self::Pkexec))
            .or_else(|_| which::which("please").map(|_| Self::Please))
            .map_err(Error::from)
    }
    // TODO : Implement actual `should_elevate` logic
    #[cfg(target_os = "linux")]
    pub fn should_elevate(&self) -> bool {
        true
    }
    #[cfg(target_os = "android")]
    pub fn should_elevate(&self) -> bool {
        false
    }
    pub fn elevate_cmd(&self, given_cmd: Command) -> Result<Command> {
        let mut cmd = Command::new(which::which(self.to_string())?);

        cmd.arg(given_cmd.get_program())
            .args(given_cmd.get_args());

        Ok(cmd)
    }
}

impl Display for ElevationHandler {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                Self::Doas => "doas",
                Self::Sudo => "sudo",
                Self::Gsudo => "gsudo",
                Self::Pkexec => "pkexec",
                Self::Please => "please",
            }
        )
    }
}

impl TryFrom<String> for ElevationHandler {
    type Error = Error;

    fn try_from(s: String) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "doas" => Ok(Self::Doas),
            "sudo" => Ok(Self::Sudo),
            "gsudo" => Ok(Self::Gsudo),
            "pkexec" => Ok(Self::Pkexec),
            "please" => Ok(Self::Please),
            _ => Err(Error::UnrecognizedElevationHandlerName(s)),
        }
    }
}
