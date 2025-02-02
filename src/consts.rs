use crate::Distro;
use std::{
    fs,
    sync::LazyLock,
};

/// Compile-time constants
pub const SKIM_PREVIEW_WINDOW: &str = "right:66%:wrap";
pub static DISTRO_BINARY_MAP: LazyLock<[(Distro, String); 4]> = LazyLock::new(|| {
    [
        (Distro::Void, "/usr/bin/xbps-install".to_string()),
        (Distro::Arch, "/usr/bin/pacman".to_string()),
        (Distro::Debian, format!("{}/usr/bin/apt", *FS_ROOT)),
        (Distro::OpenSuse, "/usr/bin/zypper".to_string()),
    ]
});

/// Run-time statics
pub static IS_TERMUX: LazyLock<bool> =
    LazyLock::new(|| fs::exists("/data/data/com.termux/").unwrap_or(false));
pub static FS_ROOT: LazyLock<&str> = LazyLock::new(|| {
    if *IS_TERMUX {
        "/data/data/com.termux/files/"
    } else {
        "/"
    }
});
