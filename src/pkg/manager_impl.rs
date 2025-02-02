use crate::{
    err::Result,
    sudo::ElevationHandler,
};

pub trait PackageManagerImpl {
    fn installed_package_list(&self) -> Result<Vec<String>>;
    fn available_package_list(&self) -> Result<Vec<String>>;
    fn interactive_install(
        &self, packages: &[String], elevation: Option<ElevationHandler>,
    ) -> Result<()>;
    fn interactive_remove(
        &self, packages: &[String], elevation: Option<ElevationHandler>,
    ) -> Result<()>;
    fn package_query_cmd(&self) -> Result<String>;
}
