mod project;

use project::Project;
use rfd::FileDialog;
use slint::{ModelRc, SharedString, VecModel};
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let app = MainWindow::new()?;

    let active_project = Rc::new(RefCell::new(None::<Project>));

    {
        let app_weak = app.as_weak();
        let active_project = Rc::clone(&active_project);
        app.on_open_project_requested(move || {
            let Some(path) = FileDialog::new().pick_folder() else {
                return;
            };

            match Project::open(&path) {
                Ok(project) => {
                    let entries = match project.entries() {
                        Ok(entries) => entries,
                        Err(error) => {
                            if let Some(app) = app_weak.upgrade() {
                                app.set_error_message(
                                    format!("Could not read the selected folder: {error}").into(),
                                );
                            }
                            return;
                        }
                    };
                    if let Some(app) = app_weak.upgrade() {
                        set_project_view(&app, &project, entries);
                        app.set_error_message(SharedString::default());
                    }
                    *active_project.borrow_mut() = Some(project);
                }
                Err(error) => {
                    if let Some(app) = app_weak.upgrade() {
                        app.set_error_message(format!("Could not open folder: {error}").into());
                    }
                }
            }
        });
    }

    {
        let app_weak = app.as_weak();
        let active_project = Rc::clone(&active_project);
        app.on_entry_selected(move |path, is_directory| {
            if !is_directory {
                if let Some(app) = app_weak.upgrade() {
                    app.set_error_message(
                        "File opening will be available in the next editing slice.".into(),
                    );
                }
                return;
            }

            let mut active_project = active_project.borrow_mut();
            let Some(project) = active_project.as_mut() else {
                return;
            };
            match project.enter_directory(Path::new(path.as_str())) {
                Ok(entries) => {
                    if let Some(app) = app_weak.upgrade() {
                        set_project_view(&app, project, entries);
                        app.set_error_message(SharedString::default());
                    }
                }
                Err(error) => {
                    if let Some(app) = app_weak.upgrade() {
                        app.set_error_message(format!("Could not open folder: {error}").into());
                    }
                }
            }
        });
    }

    {
        let app_weak = app.as_weak();
        let active_project = Rc::clone(&active_project);
        app.on_parent_requested(move || {
            let mut active_project = active_project.borrow_mut();
            let Some(project) = active_project.as_mut() else {
                return;
            };
            match project.parent_directory() {
                Ok(entries) => {
                    if let Some(app) = app_weak.upgrade() {
                        set_project_view(&app, project, entries);
                        app.set_error_message(SharedString::default());
                    }
                }
                Err(error) => {
                    if let Some(app) = app_weak.upgrade() {
                        app.set_error_message(format!("Could not refresh folder: {error}").into());
                    }
                }
            }
        });
    }

    app.run()
}

fn set_project_view(app: &MainWindow, project: &Project, entries: Vec<project::ProjectEntry>) {
    let entries = entries
        .into_iter()
        .map(|entry| ProjectEntry {
            name: entry.name.into(),
            path: entry.path.to_string_lossy().into_owned().into(),
            is_directory: entry.is_directory,
        })
        .collect::<Vec<_>>();

    app.set_project_name(
        project
            .root()
            .file_name()
            .map_or_else(
                || project.root().display().to_string(),
                |name| name.to_string_lossy().into_owned(),
            )
            .into(),
    );
    app.set_project_root(project.root().display().to_string().into());
    app.set_current_directory(project.current_directory().display().to_string().into());
    app.set_entries(ModelRc::new(VecModel::from(entries)));
    app.set_has_parent(project.current_directory() != project.root());
}
