#[easy_ext::ext(PathExt)]
impl std::path::Path {
    fn is_directory_path(&self) -> bool {
        self.to_string_lossy().ends_with('/')
    }
}
