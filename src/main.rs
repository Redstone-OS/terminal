//! # Terminal - Emulador de Terminal para RedstoneOS
//!
//! Aplicativo de terminal gráfico com:
//! - Decorações de janela (barra de título, botões)
//! - Renderização de texto com fonte bitmap
//! - Cursor piscante
//! - Buffer de scroll

#![no_std]
#![no_main]

extern crate alloc;

mod render;
mod state;
mod ui;

use redpowder::println;
use redpowder::window::Window;
use ui::window::TerminalWindow;

/// Alocador global
#[global_allocator]
static ALLOCATOR: redpowder::mem::heap::SyscallAllocator = redpowder::mem::heap::SyscallAllocator;

/// Dimensões padrão da janela
const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;

/// Intervalo entre frames (ms)
const FRAME_INTERVAL: u64 = 16;

/// Ponto de entrada
#[no_mangle]
#[link_section = ".text._start"]
pub extern "C" fn _start() -> ! {
    println!("[Terminal] Iniciando...");

    match run() {
        Ok(()) => println!("[Terminal] Encerrado normalmente"),
        Err(e) => println!("[Terminal] Erro fatal: {:?}", e),
    }

    // Loop infinito de fallback
    loop {
        let _ = redpowder::time::sleep(1000);
    }
}

/// Função principal
fn run() -> Result<(), redpowder::syscall::SysError> {
    // Criar janela no compositor
    let mut window = Window::create(100, 50, WINDOW_WIDTH, WINDOW_HEIGHT, "Terminal")?;
    println!("[Terminal] Janela criada: ID {}", window.id);

    // Criar terminal
    let mut terminal = TerminalWindow::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    println!(
        "[Terminal] Terminal inicializado: {}x{} caracteres",
        terminal.cols(),
        terminal.rows()
    );

    // Mensagem de boas-vindas
    terminal.writeln("RedstoneOS Terminal v0.1.0");
    terminal.writeln("---------------------------");
    terminal.writeln("");
    terminal.writeln("Bem-vindo ao terminal do RedstoneOS!");
    terminal.writeln("Este e um emulador de terminal grafico.");
    terminal.writeln("");
    terminal.write("redstone@localhost:~$ ");

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

        // Atualizar animações (cursor piscante sempre marca como dirty para piscar)
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
    loop {}
}
