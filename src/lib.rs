pub mod traits;
use std::{
    fs,
    iter::zip,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub enum BootstrapFileError {
    GetParent,
    ReadSourcePath(std::io::Error),
    CreateDirStructure(std::io::Error),
    BootstrapBinary(std::io::Error),
}

impl std::fmt::Display for BootstrapFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BootstrapFileError::ReadSourcePath(e) => write!(f, "Error reading source path: {}", e),
            BootstrapFileError::GetParent => write!(f, "Error getting parent directory"),
            BootstrapFileError::CreateDirStructure(e) => {
                write!(f, "Error creating directory structure: {}", e)
            }
            BootstrapFileError::BootstrapBinary(e) => {
                write!(f, "Error bootstrapping binary: {}", e)
            }
        }
    }
}

impl std::error::Error for BootstrapFileError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            BootstrapFileError::ReadSourcePath(e) => Some(e),
            BootstrapFileError::CreateDirStructure(e) => Some(e),
            BootstrapFileError::BootstrapBinary(e) => Some(e),
            _ => None,
        }
    }
}

fn create_dir_structure<P: AsRef<Path>>(path: P) -> Result<(), BootstrapFileError> {
    fs::create_dir_all(path).map_err(BootstrapFileError::CreateDirStructure)
}

pub fn copy_file<S, D>(src: S, dst: D) -> Result<u64, BootstrapFileError>
where
    S: AsRef<Path>,
    D: AsRef<Path>,
{
    dst.as_ref()
        .parent()
        .ok_or(BootstrapFileError::GetParent)
        .and_then(create_dir_structure)?;

    fs::copy(src, dst).map_err(BootstrapFileError::BootstrapBinary)
}

pub fn copy_pairs<S, D>(pairs: Vec<(S, D)>) -> Result<Vec<u64>, BootstrapFileError>
where
    S: AsRef<Path>,
    D: AsRef<Path>,
{
    pairs.iter().map(|(from, to)| copy_file(from, to)).collect()
}

fn is_executable_file<T: AsRef<Path>>(path: T) -> bool {
    fs::metadata(path)
        .map(|metadata| metadata.permissions().mode())
        .map(|mode| mode & 0o111 != 0)
        .unwrap_or(false)
}

pub fn get_executable_files<T: AsRef<Path>>(path: T) -> Result<Vec<PathBuf>, BootstrapFileError> {
    let entries = fs::read_dir(path)
        .map_err(BootstrapFileError::ReadSourcePath)?
        .filter_map(|result| result.ok())
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .filter(|file| is_executable_file(file))
        .collect::<Vec<_>>();

    Ok(entries)
}

pub fn get_filenames<T: AsRef<Path>>(paths: &[T]) -> Vec<String> {
    paths
        .iter()
        .filter_map(|path| {
            path.as_ref()
                .file_name()
                .map(|name| name.to_string_lossy().to_string())
        })
        .collect()
}

pub fn build_paths<D: AsRef<Path>>(filenames: &[String], build_path: D) -> Vec<String> {
    let build_path = build_path.as_ref().to_string_lossy();
    filenames
        .iter()
        .map(|f| format!("{build_path}/{f}/bootstrap"))
        .collect()
}

pub fn create_source_destination_pairs<D: AsRef<Path>>(
    paths: Vec<String>,
    build_path: D,
) -> Vec<(String, String)> {
    let filenames = get_filenames(&paths);
    let build_files = build_paths(&filenames, build_path.as_ref());
    zip(paths, build_files).collect::<Vec<(String, String)>>()
}
