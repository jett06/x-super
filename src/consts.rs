use crate::Cli;
use argh::FromArgs;
use std::sync::LazyLock;

pub static HELP_TEXT: LazyLock<String> = LazyLock::new(|| {
    Cli::from_args(&["x-super"], &["--help"])
        .unwrap_err()
        .output
});
pub const SKIM_PREVIEW_WINDOW: &str = "right:66%:wrap";
#[cfg(target_os = "linux")]
pub const OS_RELEASE_PATH: &str = "/etc/os-release";
