use crate::view::MainViewer;

mod data;
mod view;

fn main() {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Node Viewer",
        options,
        Box::new(|_cc| Ok(Box::new(MainViewer::default()))));
}