use eframe::egui;
use sysinfo::{
    System,
};



fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Contador",
        native_options,
        Box::new(|_cc| Ok(Box::new(MyAppData::default()))),
    )
}

// Cerebro de toda a minha operação.
struct MyAppData {
    counter: i32,
    nome_usuario: String,
}

impl Default for MyAppData {
    fn default() -> Self {
        Self { counter: 0 , nome_usuario: String::new() }
    }
}

impl eframe::App for MyAppData {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut sys = System::new_all();
            sys.refresh_all();
            // Titulo do programa.
            ui.heading("Contador interativo");

            // Pedido do nome de usuário.
            ui.group(|ui| {
                ui.label("Nome usuario: ");
                ui.text_edit_singleline(&mut self.nome_usuario);
            });

            ui.add_space(10.0);

            // Verificação se o usuário pode ou não usar os botões
            let can_click = !self.nome_usuario.is_empty();

            // Lógica do contador
            ui.horizontal(|ui|{
                // Criação de uma variável para verificar se o nome já foi preenchido para permitir adicionar ou subtrair no contador
                let botao_menos = ui.add_enabled(can_click, egui::Button::new("-"));

                if botao_menos.clicked() {
                    self.counter -= 1;
                }

                // Impressão do valor atual do contador
                ui.label(self.counter.to_string());

                let botao_mais = ui.add_enabled(can_click, egui::Button::new("+"));
                if botao_mais.clicked(){
                    self.counter += 1;
                }
            });
            ui.horizontal(|ui|{
                ui.label("Numeros de CPU:   ");
                ui.label(sys.cpus().len().to_string());
            })

        });

    }
}
