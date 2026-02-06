use eframe::egui;
use sysinfo::System;
use crate::ui_logic;

pub struct MyAppData {
    pub sys: System,
    pub show_window: bool,
}

impl Default for MyAppData {
    fn default() -> Self {
        Self {
            sys: System::new_all(),
            show_window: true,
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

        egui::Window::new("Informações do sistema: ")
            .default_width(320.0)
            .default_height(480.0)
            .open(&mut self.show_window)
            .resizable(true)
            .scroll(false)
            .show(ctx, |ui| {
                ui_logic::window_sys_info(ui, &self.sys);
            });

        ctx.request_repaint_after(std::time::Duration::from_millis(500));

    }
}