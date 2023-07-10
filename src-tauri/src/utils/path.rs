use std::path::{Path, PathBuf};

pub fn full_path(f: &str) -> PathBuf {
    // current file
    let mut path = std::env::current_dir().unwrap();
    path.push(Path::new(f));

    path
}
