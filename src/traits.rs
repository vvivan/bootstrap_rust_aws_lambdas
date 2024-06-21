use std::{
    io,
    path::{Path, PathBuf},
};

pub trait Filesystem {
    fn read_dir(path: &Path) -> Result<Vec<PathBuf>, io::Error>;
    fn is_file(path: &Path) -> bool;
    fn is_executable(path: &Path) -> bool;
}
