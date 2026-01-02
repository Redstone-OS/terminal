//! # Terminal Window
//!
//! Janela principal do terminal que integra todos os componentes.

use crate::render::colors;
use crate::render::font::{CHAR_HEIGHT, CHAR_WIDTH};
use crate::render::text::TextRenderer;
use crate::state::terminal::TerminalState;
use crate::ui::decorations::{WindowDecorations, BUTTON_WIDTH, CONTENT_PADDING, TITLE_BAR_HEIGHT};
use redpowder::window::Window;

use redpowder::event::{event_type, Event};
use redpowder::input::KeyCode;

/// Janela do terminal
pub struct TerminalWindow {
    /// Estado do terminal (buffer, cursor, etc)
    pub state: TerminalState,
    /// Decorações da janela
    pub decorations: WindowDecorations,
    /// Renderizador de texto
    renderer: TextRenderer,
    /// Largura da janela em pixels
    width: u32,
    /// Altura da janela em pixels
    height: u32,
    /// Shift pressionado
    shift: bool,
    /// Janela deve fechar
    pub should_close: bool,
    /// Flag para evitar redesenho desnecessário (flicker)
    pub dirty: bool,
}

impl TerminalWindow {
    /// Cria nova janela do terminal
    pub fn new(width: u32, height: u32) -> Self {
        // Calcular área de conteúdo disponível
        let content_width = width - 2 - (CONTENT_PADDING * 2);
        let content_height = height - TITLE_BAR_HEIGHT - 1 - (CONTENT_PADDING * 2);

        // Calcular colunas e linhas disponíveis
        let cols = content_width / CHAR_WIDTH;
        let rows = content_height / CHAR_HEIGHT;

        Self {
            state: TerminalState::new(cols, rows),
            decorations: WindowDecorations::new("Terminal", width, height),
            renderer: TextRenderer::new(),
            width,
            height,
            shift: false,
            should_close: false,
            dirty: true,
        }
    }

    /// Processa eventos
    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::Input(input) => {
                if input.event_type == event_type::KEY_DOWN {
                    let code = unsafe { core::mem::transmute::<u8, KeyCode>(input.param1 as u8) };

                    if code == KeyCode::Shift {
                        self.shift = true;
                        return;
                    }

                    if code == KeyCode::Backspace {
                        self.state.backspace();
                        self.dirty = true;
                        return;
                    }

                    if code == KeyCode::Enter {
                        self.process_command();
                        self.dirty = true;
                        return;
                    }

                    if let Some(c) = code.to_char(self.shift) {
                        self.state.write_char(c);
                        self.dirty = true;
                    }
                } else if input.event_type == event_type::KEY_UP {
                    let code = unsafe { core::mem::transmute::<u8, KeyCode>(input.param1 as u8) };
                    if code == KeyCode::Shift {
                        self.shift = false;
                    }
                } else if input.event_type == event_type::MOUSE_DOWN {
                    // Tratar clique em botões
                    let x = (input.param1 as u16 as i16) as i32;
                    let y = ((input.param2 >> 16) as u16 as i16) as i32;

                    redpowder::println!("[Terminal] Mouse click at ({}, {})", x, y);

                    // Botão fechar (no canto direito da title bar)
                    if y >= 0 && y < TITLE_BAR_HEIGHT as i32 {
                        // Fechar (X)
                        if x >= (self.width as i32 - BUTTON_WIDTH as i32) && x < self.width as i32 {
                            redpowder::println!("[Terminal] Botao fechar clicado!");
                            self.should_close = true;
                        }
                        // Minimizar (_)
                        else if x >= (self.width as i32 - (BUTTON_WIDTH as i32 * 2))
                            && x < (self.width as i32 - BUTTON_WIDTH as i32)
                        {
                            redpowder::println!("[Terminal] Botao minimizar clicado!");
                            // Por enquanto vamos apenas printar, pois precisamos de suporte no Compositor/Shell
                        }
                    }
                }
            }
            _ => {}
        }
    }

    /// Processa o comando digitado na linha atual
    fn process_command(&mut self) {
        // Obter comando da linha atual
        let line_idx = self.state.scroll_offset + self.state.cursor_y as usize;
        let line_content = self.state.lines[line_idx].clone();

        // Extrair apenas o que vem depois do prompt "redstone@localhost:~$ "
        let prompt = "redstone@localhost:~$ ";
        let cmd = if line_content.starts_with(prompt) {
            &line_content[prompt.len()..]
        } else {
            &line_content
        }
        .trim();

        self.state.write_char('\n');

        if cmd.is_empty() {
            // Nada
        } else if cmd == "help" {
            self.state.write_line("Comandos disponiveis:");
            self.state.write_line("  help  - Mostra esta lista");
            self.state.write_line("  clear - Limpa a tela");
            self.state.write_line("  exit  - Fecha o terminal");
            self.state.write_line("  ver   - Mostra versao do SO");
            self.state.write_line("");
        } else if cmd == "clear" {
            self.state.clear();
        } else if cmd == "exit" {
            self.should_close = true;
            return;
        } else if cmd == "ver" {
            self.state.write_line("RedstoneOS v0.1.0 (Firefly)");
        } else {
            self.state.write_str("Comando nao encontrado: ");
            self.state.write_line(cmd);
        }

        self.state.write_str("redstone@localhost:~$ ");
    }

    /// Escreve texto no terminal
    pub fn write(&mut self, text: &str) {
        self.state.write_str(text);
    }

    /// Escreve linha no terminal
    pub fn writeln(&mut self, text: &str) {
        self.state.write_line(text);
    }

    /// Limpa o terminal
    pub fn clear(&mut self) {
        self.state.clear();
    }

    /// Atualiza animações (cursor piscante)
    pub fn tick(&mut self) {
        self.state.tick();
    }

    /// Desenha todo o terminal na janela
    pub fn draw(&self, window: &mut Window) {
        // 1. Desenhar decorações (barra de título, borda)
        self.decorations.draw(window);

        // 2. Preencher área de conteúdo com fundo
        let (content_x, content_y, content_w, content_h) = self.decorations.content_area();
        window.fill_rect(
            content_x,
            content_y,
            content_w,
            content_h,
            colors::BACKGROUND,
        );

        // 3. Desenhar linhas de texto
        self.draw_content(window, content_x, content_y, content_w, content_h);

        // 4. Desenhar cursor
        if self.state.cursor_visible {
            self.draw_cursor(window, content_x, content_y);
        }
    }

    /// Desenha o conteúdo do terminal (linhas de texto)
    fn draw_content(&self, window: &mut Window, x: u32, y: u32, w: u32, h: u32) {
        for row in 0..self.state.rows {
            if let Some(line) = self.state.get_visible_line(row) {
                let line_y = y + (row * CHAR_HEIGHT);

                // Não desenhar fora da área
                if line_y + CHAR_HEIGHT > y + h {
                    break;
                }

                self.renderer.draw_string(window, x, line_y, line);
            }
        }
    }

    /// Desenha o cursor
    fn draw_cursor(&self, window: &mut Window, content_x: u32, content_y: u32) {
        let cursor_x = content_x + (self.state.cursor_x * CHAR_WIDTH);
        let cursor_y = content_y + (self.state.cursor_y * CHAR_HEIGHT);

        // Cursor estilo bloco
        window.fill_rect(cursor_x, cursor_y, CHAR_WIDTH, CHAR_HEIGHT, colors::CURSOR);
    }

    /// Retorna número de colunas
    pub fn cols(&self) -> u32 {
        self.state.cols
    }

    /// Retorna número de linhas
    pub fn rows(&self) -> u32 {
        self.state.rows
    }
}
