use crate::err::Result;

pub trait PackageManagerImpl {
    fn installed_package_list(&self) -> Result<Vec<String>>;
    fn available_package_list(&self) -> Result<Vec<String>>;
    fn interactive_install(&self, packages: &[String]) -> Result<()>;
    fn interactive_remove(&self, packages: &[String]) -> Result<()>;
    fn package_query_cmd(&self) -> Result<String>;
}
