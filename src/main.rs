mod app;
mod ui_logic;

use app::MyAppData;
fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Informações do Sistema",
        native_options,
        Box::new(|_cc| Ok(Box::new(MyAppData::default()))),
    )
}

