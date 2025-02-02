use crate::Cli;
use argh::FromArgs;
use std::{
    fs,
    sync::LazyLock,
};

pub static HELP_TEXT: LazyLock<String> = LazyLock::new(|| {
    Cli::from_args(&["x-super"], &["--help"])
        .unwrap_err()
        .output
});
pub const SKIM_PREVIEW_WINDOW: &str = "right:66%:wrap";
pub static IS_TERMUX: LazyLock<bool> =
    LazyLock::new(|| fs::exists("/data/data/com.termux/").unwrap_or(false));
pub const OS_RELEASE_PATH: &str = "/etc/os-release";
