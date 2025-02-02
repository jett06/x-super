use crate::Distro;

pub const DISTRO_BINARY_MAP: [(Distro, &str); 5] = [
    (Distro::Void, "/usr/bin/xbps-install"),
    (Distro::Arch, "/usr/bin/pacman"),
    (Distro::Debian, "/usr/bin/apt"),
    (Distro::Debian, "/data/data/com.termux/files/usr/bin/apt"),
    (Distro::OpenSuse, "/usr/bin/zypper"),
];
pub const SKIM_PREVIEW_WINDOW: &str = "right:66%:wrap";
