use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_TEMP_ID: AtomicUsize = AtomicUsize::new(0);

struct TempDirectory(PathBuf);

impl TempDirectory {
    fn new() -> Self {
        let id = NEXT_TEMP_ID.fetch_add(1, Ordering::Relaxed);
        let path =
            std::env::temp_dir().join(format!("cockpit-project-test-{}-{id}", std::process::id()));
        fs::create_dir_all(&path).expect("temporary directory should be created");
        Self(path)
    }
}

impl Drop for TempDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn opens_regular_directory_and_lists_directories_first_with_stable_names() {
    let temp = TempDirectory::new();
    fs::create_dir(temp.0.join("z-folder")).unwrap();
    fs::write(temp.0.join("b.txt"), "text").unwrap();
    fs::write(temp.0.join("a.txt"), "text").unwrap();
    fs::write(temp.0.join("A.txt"), "text").unwrap();

    let project = Project::open(&temp.0).unwrap();
    let entries = project.entries().unwrap();

    assert_eq!(entries[0].name, "z-folder");
    assert!(entries[0].is_directory);
    assert_eq!(entries[1].name, "A.txt");
    assert_eq!(entries[2].name, "a.txt");
    assert_eq!(entries[3].name, "b.txt");
}

#[test]
fn rejects_file_as_project_root() {
    let temp = TempDirectory::new();
    let file = temp.0.join("file.txt");
    fs::write(&file, "text").unwrap();

    assert_eq!(
        Project::open(&file).unwrap_err().kind(),
        io::ErrorKind::InvalidInput
    );
}

#[test]
fn navigates_inside_project_and_returns_to_root() {
    let temp = TempDirectory::new();
    let child = temp.0.join("child");
    fs::create_dir(&child).unwrap();
    let mut project = Project::open(&temp.0).unwrap();

    project.enter_directory(&child).unwrap();
    assert_eq!(project.current_directory(), child.canonicalize().unwrap());
    project.parent_directory().unwrap();
    assert_eq!(project.current_directory(), project.root());
}

#[test]
fn rejects_navigation_outside_project() {
    let temp = TempDirectory::new();
    let outside = TempDirectory::new();
    let mut project = Project::open(&temp.0).unwrap();

    assert_eq!(
        project.enter_directory(&outside.0).unwrap_err().kind(),
        io::ErrorKind::PermissionDenied
    );
    assert_eq!(project.current_directory(), project.root());
}

#[cfg(unix)]
#[test]
fn omits_symlinks_from_project_entries() {
    use std::os::unix::fs::symlink;

    let temp = TempDirectory::new();
    let outside = TempDirectory::new();
    fs::write(outside.0.join("external.txt"), "text").unwrap();
    symlink(&outside.0, temp.0.join("external-folder-link")).unwrap();

    let project = Project::open(&temp.0).unwrap();
    let entries = project.entries().unwrap();

    assert_eq!(entries.len(), 0);
}
