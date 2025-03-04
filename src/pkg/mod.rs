pub mod manager_impl;
#[cfg(any(target_os = "linux", target_os = "android"))]
use crate::{
    err::Result,
    linux::LinuxDistro,
};
pub use manager_impl::*;

#[cfg(any(target_os = "linux", target_os = "android"))]
pub fn new_package_manager() -> Result<impl PackageManagerImpl> {
    LinuxDistro::try_from_env()
}

#[cfg(target_os = "windows")]
pub fn new_package_manager() -> Result<impl PackageManagerImpl> {
    todo!()
}
