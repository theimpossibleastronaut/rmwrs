use std::ffi::OsStr;
use std::path::Path;

pub fn get_basename(path: &Path) -> &OsStr {
    path.file_name().unwrap_or_else(|| path.as_os_str())
}
