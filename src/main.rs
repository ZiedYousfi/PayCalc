mod gui;
mod utils;

fn main() {
    println!("Hello, world!");

    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "PayCalc",
        options,
        Box::new(|cc| Ok(Box::new(gui::MyEguiApp::new(cc)))),
    );
}
