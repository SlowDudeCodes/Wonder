use eframe::egui;
use walkdir::WalkDir;
use std::path::{PathBuf};

// Structure to hold the file explorer state
struct FileExplorer {
    current_path: PathBuf, // Current directory path
    entries: Vec<PathBuf>, // List of files and directories
}

impl Default for FileExplorer {
    fn default() -> Self {
        let start_path = std::env::current_dir().unwrap();  // Start in the current directory
        Self {
            current_path: start_path.clone(),
            entries: read_directory(&start_path), // Read the initial directory
        }
    }
}

// Helper function to read the content of a directory
fn read_directory(path: &PathBuf) -> Vec<PathBuf> {
    let mut entries = Vec::new();
    for entry in WalkDir::new(path)
        .max_depth(1) // Only read the current directory
        .into_iter()
        .filter_map(|e| e.ok()) // Filter out errors
    {
        entries.push(entry.path().to_path_buf());
    }
    entries
}

impl FileExplorer {
    // Navigate to a new directory and update the file list
    fn change_directory(&mut self, new_path: PathBuf) {
        self.current_path = new_path.clone();
        self.entries = read_directory(&new_path);
    }

    // Navigate to the parent directory
    fn go_up(&mut self) {
        if let Some(parent) = self.current_path.parent() {
            self.change_directory(parent.to_path_buf());
        }
    }
}

impl eframe::App for FileExplorer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("File Explorer");

            // Show the current path
            ui.horizontal(|ui| {
                ui.label("Current Path: ");
                ui.monospace(self.current_path.display().to_string());
            });

            // Button to go up one directory
            if ui.button("Up").clicked() {
                self.go_up();
            }

            // Variable to store the new directory to change to, if needed
            let mut new_directory: Option<PathBuf> = None;

            // List the files and directories
            egui::ScrollArea::vertical().show(ui, |ui| {
                for entry in &self.entries {
                    let file_name = entry.file_name().unwrap_or_default().to_string_lossy();
                    if entry.is_dir() {
                        if ui.button(format!("[Dir] {}", file_name)).clicked() {
                            // Store the directory to navigate into
                            new_directory = Some(entry.clone());
                        }
                    } else {
                        ui.label(file_name);
                    }
                }
            });

            // After the loop, change the directory if one was clicked
            if let Some(dir) = new_directory {
                self.change_directory(dir);
            }
        });
    }
}


fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "File Explorer",
        options,
        Box::new(|_cc| Box::new(FileExplorer::default())),
    )
}
