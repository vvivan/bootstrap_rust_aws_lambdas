pub mod traits;
use std::{
    fs, io,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub enum CopyFileError {
    GetParent,
    CreateDirStructure(std::io::Error),
    BootstrapBinary(std::io::Error),
}

impl std::fmt::Display for CopyFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CopyFileError::GetParent => write!(f, "Error getting parent directory"),
            CopyFileError::CreateDirStructure(e) => {
                write!(f, "Error creating directory structure: {}", e)
            }
            CopyFileError::BootstrapBinary(e) => write!(f, "Error bootstrapping binary: {}", e),
        }
    }
}

impl std::error::Error for CopyFileError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CopyFileError::CreateDirStructure(e) => Some(e),
            CopyFileError::BootstrapBinary(e) => Some(e),
            _ => None,
        }
    }
}

fn create_dir_structure<P: AsRef<Path>>(path: P) -> Result<(), CopyFileError> {
    println!("Creating directory: {:?}", path.as_ref());

    fs::create_dir_all(path).map_err(CopyFileError::CreateDirStructure)
}

pub fn copy_file<S, D>(src: S, dst: D) -> Result<u64, CopyFileError>
where
    S: AsRef<Path>,
    D: AsRef<Path>,
{
    dst.as_ref()
        .parent()
        .ok_or(CopyFileError::GetParent)
        .and_then(create_dir_structure)?;

    println!("Copying {:?} to {:?}", src.as_ref(), dst.as_ref());
    fs::copy(src, dst).map_err(CopyFileError::BootstrapBinary)
}

pub fn get_executable_files(path: &String) -> Result<Vec<PathBuf>, io::Error> {
    let entries = fs::read_dir(path)?
        .filter_map(|res| res.ok())
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .filter(|file| {
            fs::metadata(file)
                .map(|metadata| metadata.permissions().mode())
                .map(|mode| mode & 0o111 != 0)
                .unwrap_or(false)
        })
        .collect::<Vec<_>>();

    Ok(entries)
}

pub fn get_filenames(paths: &[PathBuf]) -> Vec<String> {
    paths
        .iter()
        .filter_map(|path| {
            path.file_name()
                .map(|name| name.to_string_lossy().to_string())
        })
        .collect()
}

pub fn build_paths(filenames: &[String], build_path: &str) -> Vec<String> {
    filenames
        .iter()
        .map(|f| format!("{build_path}/{f}/bootstrap"))
        .collect()
}
