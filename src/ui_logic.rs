use eframe::egui;
use sysinfo::System;

// Formatação de Bytes para Giga kb ou mb
fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

pub fn render_sys_info(ui: &mut egui::Ui, sys: &System) {
    // Centralização e estilização do titulo da página
    ui.vertical_centered(|ui| {
        ui.heading(
            egui::RichText::new("System Monitor")
            .size(32.0)
            .color(egui::Color32::from_rgb(102, 255, 255))
        );
    });

    ui.add_space(20.0);

    // Informações do CPU
    ui.horizontal(|ui| {
        ui.colored_label(egui::Color32::RED, "CPU Cores: ");
        ui.colored_label(egui::Color32::WHITE, sys.cpus().len().to_string());
    });

    // Espaçamento de cpu para memoria
    ui.add_space(10.0);
    ui.separator();
    ui.add_space(10.0);

    // Informações da memoria ram
    ui.horizontal(|ui| {
        ui.colored_label(egui::Color32::RED, "Total Memory: ");
        ui.colored_label(egui::Color32::WHITE, format_bytes(sys.total_memory()))
    });

    ui.horizontal(|ui| {
        ui.colored_label(egui::Color32::RED, "Memory Usage: ");
        ui.colored_label(egui::Color32::WHITE, format_bytes(sys.used_memory()))
    });

    ui.add_space(5.0);

    let memory_usage = sys.used_memory() as f64 / sys.total_memory() as f64;
    let percentage = memory_usage * 100.0;

    let color = if percentage <= 50.0 {
        egui::Color32::GREEN
    } else if percentage <= 70.0 {
       egui::Color32::YELLOW
    } else{
       egui::Color32::RED
    };

    ui.add(
        egui::ProgressBar::new(memory_usage as f32)
            .fill(color)
            .text(format!("{:.1}%", percentage))
    );

    // Espaçamento de Ram para o próximo
    ui.add_space(10.0);
    ui.separator();
    ui.add_space(10.0);

}