#[cfg(any(target_os = "linux", target_os = "android"))]
pub mod linux;
#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(any(target_os = "linux", target_os = "android"))]
pub use linux::*;
#[cfg(target_os = "windows")]
pub use windows::*;
