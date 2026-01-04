//! # Terminal Window
//!
//! Janela principal do terminal que integra todos os componentes.

use crate::render::colors;
use crate::render::font::{CHAR_HEIGHT, CHAR_WIDTH};
use crate::render::text::TextRenderer;
use crate::shell::commands::CommandResult;
use crate::shell::{execute_command, ShellContext};
use crate::state::terminal::TerminalState;
use crate::ui::decorations::{WindowDecorations, BUTTON_WIDTH, CONTENT_PADDING, TITLE_BAR_HEIGHT};
use alloc::string::String;
use gfx_types::color::Color;
use gfx_types::geometry::Rect;
use redpowder::window::Window;

use redpowder::event::{event_type, Event};
use redpowder::input::KeyCode;

// TODO: Revisar no futuro
#[allow(unused)]
/// Janela do terminal
pub struct TerminalWindow {
    /// Estado do terminal (buffer, cursor, etc)
    pub state: TerminalState,
    /// Contexto do shell (CWD, usuário, etc)
    pub shell_ctx: ShellContext,
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
    /// Buffer do comando atual (linha de input)
    input_buffer: String,
    /// Posição do prompt na linha atual
    prompt_pos: usize,
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

        let shell_ctx = ShellContext::new();

        Self {
            state: TerminalState::new(cols, rows),
            shell_ctx,
            decorations: WindowDecorations::new("Terminal", width, height),
            renderer: TextRenderer::new(),
            width,
            height,
            shift: false,
            should_close: false,
            dirty: true,
            input_buffer: String::new(),
            prompt_pos: 0,
        }
    }

    /// Mostra mensagem de boas-vindas
    pub fn show_welcome(&mut self) {
        self.state.write_line("RedstoneOS Terminal v0.2.0");
        self.state.write_line("==========================");
        self.state.write_line("");
        self.state
            .write_line("Bem-vindo ao terminal do RedstoneOS!");
        self.state
            .write_line("Digite 'help' para ver os comandos disponiveis.");
        self.state.write_line("");
        self.show_prompt();
    }

    /// Mostra o prompt
    fn show_prompt(&mut self) {
        let prompt = self.shell_ctx.prompt();
        self.state.write_str(&prompt);
        self.prompt_pos = prompt.len();
        self.input_buffer.clear();
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
                        self.handle_backspace();
                        self.dirty = true;
                        return;
                    }

                    if code == KeyCode::Enter {
                        self.handle_enter();
                        self.dirty = true;
                        return;
                    }

                    if let Some(c) = code.to_char(self.shift) {
                        self.handle_char(c);
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

                    // Botão fechar (no canto direito da title bar)
                    if y >= 0 && y < TITLE_BAR_HEIGHT as i32 {
                        // Fechar (X)
                        if x >= (self.width as i32 - BUTTON_WIDTH as i32) && x < self.width as i32 {
                            self.should_close = true;
                        }
                    }
                }
            }
            _ => {}
        }
    }

    /// Trata caractere digitado
    fn handle_char(&mut self, c: char) {
        self.input_buffer.push(c);
        self.state.write_char(c);
    }

    /// Trata backspace
    fn handle_backspace(&mut self) {
        if !self.input_buffer.is_empty() {
            self.input_buffer.pop();
            self.state.backspace();
        }
    }

    /// Trata Enter - executa comando
    fn handle_enter(&mut self) {
        self.state.write_char('\n');

        // Executar comando
        let cmd = self.input_buffer.clone();
        match execute_command(&cmd, &mut self.shell_ctx, &mut self.state) {
            CommandResult::Ok => {
                self.show_prompt();
            }
            CommandResult::Exit => {
                self.should_close = true;
            }
            CommandResult::Clear => {
                self.state.clear();
                self.show_prompt();
            }
            CommandResult::Error(msg) => {
                self.state.write_str("Erro: ");
                self.state.write_line(&msg);
                self.show_prompt();
            }
        }
    }

    // TODO: Revisar no futuro
    #[allow(unused)]
    /// Escreve texto no terminal
    pub fn write(&mut self, text: &str) {
        self.state.write_str(text);
    }

    // TODO: Revisar no futuro
    #[allow(unused)]
    /// Escreve linha no terminal
    pub fn writeln(&mut self, text: &str) {
        self.state.write_line(text);
    }

    // TODO: Revisar no futuro
    #[allow(unused)]
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
        let content_rect = Rect::new(content_x as i32, content_y as i32, content_w, content_h);
        window.fill_rect(content_rect, Color(colors::BACKGROUND));

        // 3. Desenhar linhas de texto
        self.draw_content(window, content_x, content_y, content_w, content_h);

        // 4. Desenhar cursor
        if self.state.cursor_visible {
            self.draw_cursor(window, content_x, content_y);
        }
    }

    // TODO: Revisar no futuro
    #[allow(unused)]
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
        let cursor_rect = Rect::new(cursor_x as i32, cursor_y as i32, CHAR_WIDTH, CHAR_HEIGHT);
        window.fill_rect(cursor_rect, Color(colors::CURSOR));
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
