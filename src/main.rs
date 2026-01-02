//! # Terminal - Emulador de Terminal para RedstoneOS
//!
//! Aplicativo de terminal gráfico com:
//! - Shell interativo com comandos reais
//! - Decorações de janela (barra de título, botões)
//! - Renderização de texto com fonte bitmap
//! - Cursor piscante
//! - Buffer de scroll
//!
//! ## Comandos Suportados
//!
//! | Comando                   | Status         |
//! |---------------------------|----------------|
//! | ls, cd, pwd, cat, tree    | 🟢 Funcional   |
//! | mkdir, rmdir, rm, cp, mv  | ⚪ Stub        |
//! | help, clear, exit, ver    | 🟢 Funcional   |
//! | uptime                    | 🟢 Funcional   |
//! | ps, kill, top, jobs       | ⚪ Stub        |
//! | sysinfo, meminfo          | ⚪ Stub        |

#![no_std]
#![no_main]

extern crate alloc;

mod render;
mod shell;
mod state;
mod ui;

use redpowder::println;
use redpowder::window::Window;
use ui::window::TerminalWindow;

/// Alocador global
#[global_allocator]
static ALLOCATOR: redpowder::mem::heap::SyscallAllocator = redpowder::mem::heap::SyscallAllocator;

/// Dimensões padrão da janela
const WINDOW_WIDTH: u32 = 720;
const WINDOW_HEIGHT: u32 = 480;

/// Ponto de entrada
#[no_mangle]
#[link_section = ".text._start"]
pub extern "C" fn _start() -> ! {
    println!("[Terminal] Iniciando v0.2.0...");

    match run() {
        Ok(()) => println!("[Terminal] Encerrado normalmente"),
        Err(e) => println!("[Terminal] Erro fatal: {:?}", e),
    }

    // Exit limpo
    redpowder::process::exit(0);
}

/// Função principal
fn run() -> Result<(), redpowder::syscall::SysError> {
    // Criar janela no compositor
    let mut window = Window::create(80, 50, WINDOW_WIDTH, WINDOW_HEIGHT, "Terminal")?;
    println!("[Terminal] Janela criada: ID {}", window.id);

    // Criar terminal
    let mut terminal = TerminalWindow::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    println!(
        "[Terminal] Terminal inicializado: {}x{} caracteres",
        terminal.cols(),
        terminal.rows()
    );

    // Mensagem de boas-vindas
    terminal.show_welcome();

    // Primeira renderização
    terminal.draw(&mut window);
    window.present()?;

    println!("[Terminal] Entrando no loop principal...");

    // Loop principal
    loop {
        // Processar eventos
        for event in window.poll_events() {
            terminal.handle_event(event);
        }

        if terminal.should_close {
            println!("[Terminal] Fechando janela...");
            break;
        }

        // Atualizar animações (cursor piscante)
        let old_cursor_visible = terminal.state.cursor_visible;
        terminal.tick();
        if old_cursor_visible != terminal.state.cursor_visible {
            terminal.dirty = true;
        }

        // Redesenhar apenas se necessário
        if terminal.dirty {
            terminal.draw(&mut window);

            // Enviar ao compositor
            let _ = window.present();
            terminal.dirty = false;
        }

        // Throttle para 60 FPS estável
        let _ = redpowder::time::sleep(16);
    }

    // Notificar o compositor para destruir a janela ao sair
    let _ = window.destroy();

    Ok(())
}

/// Panic handler
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("[Terminal] PANIC: {:?}", info);
    redpowder::process::exit(1);
}
