//! # Terminal Library
//!
//! Biblioteca do emulador de terminal para RedstoneOS.
//!
//! ## Módulos
//!
//! - `state`: Estado do terminal (buffer, cursor, scroll)
//! - `render`: Renderização (fonte, texto, decorações)
//! - `ui`: Componentes visuais (janela, barra de título)
//! - `shell`: Shell interativo com comandos

#![no_std]

extern crate alloc;

pub mod render;
pub mod shell;
pub mod state;
pub mod ui;

// Re-exports para conveniência
pub use render::font::BitFont;
pub use shell::{execute_command, ShellContext};
pub use state::terminal::TerminalState;
pub use ui::decorations::WindowDecorations;
pub use ui::window::TerminalWindow;
