use std::io::Result as IOResult;

pub trait PackageManagerImpl {
    fn installed_package_list(&self) -> IOResult<Vec<String>>;
    fn available_package_list(&self) -> IOResult<Vec<String>>;
    fn interactive_install(&self, packages: &[String]) -> !;
    fn interactive_remove(&self, packages: &[String]) -> !;
    fn package_query_cmd(&self) -> String;
}
