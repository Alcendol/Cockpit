use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct Project {
    root: PathBuf,
    current_directory: PathBuf,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ProjectEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_directory: bool,
}

impl Project {
    pub fn open(path: &Path) -> io::Result<Self> {
        let root = path.canonicalize()?;
        if !root.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "The selected project path is not a directory.",
            ));
        }
        if root.to_str().is_none() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "The selected project path cannot be represented by the interface.",
            ));
        }

        Ok(Self {
            current_directory: root.clone(),
            root,
        })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn current_directory(&self) -> &Path {
        &self.current_directory
    }

    pub fn entries(&self) -> io::Result<Vec<ProjectEntry>> {
        read_entries(&self.current_directory)
    }

    pub fn enter_directory(&mut self, path: &Path) -> io::Result<Vec<ProjectEntry>> {
        let candidate = path.canonicalize()?;
        if !candidate.starts_with(&self.root) {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "The selected directory is outside the active project.",
            ));
        }
        if !candidate.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "The selected project entry is not a directory.",
            ));
        }

        let entries = read_entries(&candidate)?;
        self.current_directory = candidate;
        Ok(entries)
    }

    pub fn parent_directory(&mut self) -> io::Result<Vec<ProjectEntry>> {
        if self.current_directory != self.root
            && let Some(parent) = self.current_directory.parent()
        {
            let entries = read_entries(parent)?;
            self.current_directory = parent.to_path_buf();
            return Ok(entries);
        }
        self.entries()
    }
}

fn read_entries(path: &Path) -> io::Result<Vec<ProjectEntry>> {
    let mut entries = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_type = entry.file_type()?;

        // Do not follow symlinks: they can lead outside the selected project.
        if !file_type.is_file() && !file_type.is_dir() {
            continue;
        }

        let Ok(name) = entry.file_name().into_string() else {
            continue;
        };
        entries.push(ProjectEntry {
            name,
            path: entry.path(),
            is_directory: file_type.is_dir(),
        });
    }

    entries.sort_by(|left, right| {
        right
            .is_directory
            .cmp(&left.is_directory)
            .then_with(|| left.name.to_lowercase().cmp(&right.name.to_lowercase()))
            .then_with(|| left.name.cmp(&right.name))
    });
    Ok(entries)
}

#[cfg(test)]
mod tests;
