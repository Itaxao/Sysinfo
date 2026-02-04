<div align="center">

# system-monitor-gui

🖥️ **System Monitor GUI** — Aplicação desktop em **Rust** para monitoramento de recursos do sistema operacional, desenvolvida com foco em aprendizado de GUI e conceitos de sistemas.

![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange?logo=rust)
![Platform](https://img.shields.io/badge/Platform-Windows-blue)
![Status](https://img.shields.io/badge/Status-Em%20desenvolvimento-yellow)

</div>

---

## 📌 Sobre

Este projeto é um **system monitor** simples escrito em **Rust**, utilizando **egui/eframe** para a interface gráfica e **sysinfo** para coleta de dados do sistema.

O objetivo principal é **estudo e experimentação**, explorando:

* Interfaces gráficas em Rust
* Comunicação entre lógica de sistema e GUI
* Conceitos básicos de sistemas operacionais

---

## ✨ Funcionalidades

* Exibição de uso de **CPU**
* Exibição de uso de **memória RAM**
* Informações do sistema operacional
* Atualização periódica dos dados
* Interface gráfica simples e responsiva

### Planejado

* Listagem de processos
* Detalhes de CPU/RAM por processo
* Melhor organização modular

---

## 🧠 Tecnologias Utilizadas

* **Rust**
* [`eframe`](https://crates.io/crates/eframe)
* [`egui`](https://crates.io/crates/egui)
* [`sysinfo`](https://crates.io/crates/sysinfo)

---

## 📂 Estrutura do Projeto

```text
system-monitor-gui/
├── src/
│   ├── main.rs        # Ponto de entrada da aplicação
│   ├── app.rs         # Lógica principal da aplicação
│   └── system.rs      # Coleta de informações do sistema
├── Cargo.toml
└── README.md
```

> A estrutura pode mudar conforme o projeto evolui.

---

## ▶️ Como Executar

### Pré-requisitos

* Rust (versão estável)
* Cargo

### Passos

```bash
git clone https://github.com/seu-usuario/system-monitor-gui.git
cd system-monitor-gui
cargo run
```

---

## 🪟 Plataforma

* Windows (principal)

O código pode ser adaptado futuramente para Linux e macOS.

---

## 🚧 Status

Projeto em **desenvolvimento ativo**, voltado para **fins educacionais**.

Mudanças frequentes são esperadas.

---

## 🤝 Contribuindo

Contribuições são bem-vindas!

* Abra uma *issue* para sugestões ou bugs
* Envie um *pull request* com melhorias

---

## 📄 Licença

Este projeto é distribuído **exclusivamente para fins educacionais**.

---

<div align="center">

🦀 *Aprendendo Rust na prática, explorando GUIs e sistemas operacionais.*

</div>
