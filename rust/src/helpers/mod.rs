use std::{env, ffi::OsStr, fs::{self, metadata}, io, path::{Path, PathBuf}};

fn _list_files(vec: &mut Vec<PathBuf>, path: &Path, extension: &str) -> io::Result<()> {
    if metadata(&path)?.is_dir() {
        let paths = fs::read_dir(&path)?;
        for path_result in paths {
            let full_path = path_result?.path();
            if metadata(&full_path)?.is_dir() {
                _list_files(vec, &full_path, extension)?
            } else if full_path.extension().and_then(OsStr::to_str) == Some(extension) {
                vec.push(full_path);
            }
        }
    }
    Ok(())
}

// Recursively list files
pub  fn list_files(path: &Path, extension: &str) -> io::Result<Vec<PathBuf>> {
    let mut vec = Vec::new();
    _list_files(&mut vec,&path, extension)?;
    Ok(vec)
}