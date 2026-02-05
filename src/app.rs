use eframe::egui;
use sysinfo::System;
use crate::ui_logic;

pub struct MyAppData {
    pub sys: System,
}

impl Default for MyAppData {
    fn default() -> Self {
        Self {
            sys: System::new_all(),
        }
    }
}

impl eframe::App for MyAppData {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.sys.refresh_cpu_usage();
        self.sys.refresh_memory();

        // Estilização do background do aplicativo
        let mut visuals = egui::Visuals::dark();
        visuals.panel_fill = egui::Color32::from_hex("#F06541").unwrap();
        ctx.set_visuals(visuals);

        // Chama a função UI do arquivo ui_logic.
        egui::CentralPanel::default().show(ctx, |ui| {
            ui_logic::render_sys_info(ui, &self.sys);
        });
        ctx.request_repaint_after(std::time::Duration::from_millis(500));

    }
}