mod project;

use project::Project;
use rfd::FileDialog;
use slint::{ModelRc, SharedString, VecModel};
use std::io;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let app = MainWindow::new()?;
    let active_project = Arc::new(Mutex::new(None::<Project>));
    let request_generation = Arc::new(AtomicU64::new(0));

    {
        let app_weak = app.as_weak();
        let active_project = Arc::clone(&active_project);
        let request_generation = Arc::clone(&request_generation);
        app.on_open_project_requested(move || {
            let Some(path) = FileDialog::new().pick_folder() else {
                return;
            };

            let request_id = next_request_id(&request_generation);
            start_project_operation(
                app_weak.clone(),
                Arc::clone(&active_project),
                Arc::clone(&request_generation),
                request_id,
                "Could not open folder",
                move || {
                    let project = Project::open(&path)?;
                    let entries = project.entries()?;
                    Ok((project, entries))
                },
            );
        });
    }

    {
        let app_weak = app.as_weak();
        let active_project = Arc::clone(&active_project);
        let request_generation = Arc::clone(&request_generation);
        app.on_entry_selected(move |path, is_directory| {
            let request_id = next_request_id(&request_generation);
            if !is_directory {
                if let Some(app) = app_weak.upgrade() {
                    app.set_error_message(
                        "File opening will be available in the next editing slice.".into(),
                    );
                }
                return;
            }

            let Some(mut project) = active_project
                .lock()
                .expect("active project mutex should not be poisoned")
                .clone()
            else {
                return;
            };

            start_project_operation(
                app_weak.clone(),
                Arc::clone(&active_project),
                Arc::clone(&request_generation),
                request_id,
                "Could not open folder",
                move || {
                    let entries = project.enter_directory(Path::new(path.as_str()))?;
                    Ok((project, entries))
                },
            );
        });
    }

    {
        let app_weak = app.as_weak();
        let active_project = Arc::clone(&active_project);
        let request_generation = Arc::clone(&request_generation);
        app.on_parent_requested(move || {
            let request_id = next_request_id(&request_generation);
            let Some(mut project) = active_project
                .lock()
                .expect("active project mutex should not be poisoned")
                .clone()
            else {
                return;
            };

            start_project_operation(
                app_weak.clone(),
                Arc::clone(&active_project),
                Arc::clone(&request_generation),
                request_id,
                "Could not refresh folder",
                move || {
                    let entries = project.parent_directory()?;
                    Ok((project, entries))
                },
            );
        });
    }

    app.run()
}

fn next_request_id(generation: &AtomicU64) -> u64 {
    generation.fetch_add(1, Ordering::Relaxed).wrapping_add(1)
}

fn start_project_operation<F>(
    app_weak: slint::Weak<MainWindow>,
    active_project: Arc<Mutex<Option<Project>>>,
    request_generation: Arc<AtomicU64>,
    request_id: u64,
    error_context: &'static str,
    operation: F,
) where
    F: FnOnce() -> io::Result<(Project, Vec<project::ProjectEntry>)> + Send + 'static,
{
    let _worker = thread::spawn(move || {
        let result = operation();
        if let Err(error) = slint::invoke_from_event_loop(move || {
            if request_generation.load(Ordering::Relaxed) != request_id {
                return;
            }

            let Some(app) = app_weak.upgrade() else {
                return;
            };
            match result {
                Ok((project, entries)) => {
                    set_project_view(&app, &project, entries);
                    *active_project
                        .lock()
                        .expect("active project mutex should not be poisoned") = Some(project);
                    app.set_error_message(SharedString::default());
                }
                Err(error) => {
                    app.set_error_message(format!("{error_context}: {error}").into());
                }
            }
        }) {
            eprintln!("Could not return a workspace result to the UI: {error}");
        }
    });
}

fn set_project_view(app: &MainWindow, project: &Project, entries: Vec<project::ProjectEntry>) {
    let entries = entries
        .into_iter()
        .filter_map(|entry| {
            let path = entry.path.to_str()?;
            Some(ProjectEntry {
                name: entry.name.into(),
                path: path.into(),
                is_directory: entry.is_directory,
            })
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
