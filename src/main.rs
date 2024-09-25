use eframe::App;
use egui::{Context, CentralPanel};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Wonder",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

#[derive(Default)]
struct MyApp {
    selected_label: Option<String>, 
}

impl App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.label("Select a label:");

            // Create selectable labels
            if ui.selectable_label(self.selected_label.as_ref() == Some(&"Label 1".to_string()), "Label 1").clicked() {
                self.selected_label = Some("Label 1".to_string());
            }
            if ui.selectable_label(self.selected_label.as_ref() == Some(&"Label 2".to_string()), "Label 2").clicked() {
                self.selected_label = Some("Label 2".to_string());
            }
            if ui.selectable_label(self.selected_label.as_ref() == Some(&"Label 3".to_string()), "Label 3").clicked() {
                self.selected_label = Some("Label 3".to_string());
            }

            // Show the currently selected label
            if let Some(ref label) = self.selected_label {
                ui.label(format!("You selected: {}", label));
            } else {
                ui.label("No label selected.");
            }



            /*menu::bar(ui, |ui| {
                ui.menu_button("Create", |ui| {
                    if ui.button("Folder").clicked() {
                        // …
                    }
                    if ui.button("File").clicked() {
                        // …
                    }
                    if ui.button("Shortcut").clicked() {
                        // …
                    }
                });
            });*/
           
        });
    }
}