//! # Terminal State
//!
//! Estado interno do emulador de terminal.

use alloc::string::String;
use alloc::vec::Vec;

/// Número máximo de linhas no buffer de scroll
const MAX_SCROLL_LINES: usize = 1000;

/// Estado do terminal
pub struct TerminalState {
    /// Largura em caracteres
    pub cols: u32,
    /// Altura em caracteres
    pub rows: u32,
    /// Buffer de linhas
    pub lines: Vec<String>,
    /// Posição X do cursor (coluna)
    pub cursor_x: u32,
    /// Posição Y do cursor (linha visível)
    pub cursor_y: u32,
    /// Cursor visível (para animação)
    pub cursor_visible: bool,
    /// Offset de scroll (primeira linha visível)
    pub scroll_offset: usize,
    /// Contador de frames para cursor piscante
    cursor_blink_counter: u32,
}

impl TerminalState {
    /// Cria novo estado do terminal
    pub fn new(cols: u32, rows: u32) -> Self {
        let mut lines = Vec::with_capacity(rows as usize);

        // Inicializar com linhas vazias
        for _ in 0..rows {
            lines.push(String::new());
        }

        Self {
            cols,
            rows,
            lines,
            cursor_x: 0,
            cursor_y: 0,
            cursor_visible: true,
            scroll_offset: 0,
            cursor_blink_counter: 0,
        }
    }

    /// Escreve um caractere na posição atual do cursor
    pub fn write_char(&mut self, c: char) {
        match c {
            '\n' => self.new_line(),
            '\r' => self.cursor_x = 0,
            '\x08' => self.backspace(), // Backspace
            _ => {
                // Garantir que temos linhas suficientes
                while self.lines.len() <= (self.scroll_offset + self.cursor_y as usize) {
                    self.lines.push(String::new());
                }

                let line_idx = self.scroll_offset + self.cursor_y as usize;

                // Expandir a linha se necessário
                let line = &mut self.lines[line_idx];
                while line.len() < self.cursor_x as usize {
                    line.push(' ');
                }

                // Inserir ou substituir caractere
                if self.cursor_x as usize >= line.len() {
                    line.push(c);
                } else {
                    line.replace_range(
                        self.cursor_x as usize..self.cursor_x as usize + 1,
                        &char_to_string(c),
                    );
                }

                self.cursor_x += 1;

                // Wrap se necessário
                if self.cursor_x >= self.cols {
                    self.new_line();
                }
            }
        }
    }

    /// Escreve uma string
    pub fn write_str(&mut self, s: &str) {
        for c in s.chars() {
            self.write_char(c);
        }
    }

    /// Escreve uma linha completa com quebra
    pub fn write_line(&mut self, s: &str) {
        self.write_str(s);
        self.new_line();
    }

    /// Nova linha
    fn new_line(&mut self) {
        self.cursor_x = 0;
        self.cursor_y += 1;

        // Scroll se ultrapassar área visível
        if self.cursor_y >= self.rows {
            self.scroll_offset += 1;
            self.cursor_y = self.rows - 1;

            // Adicionar linha vazia
            self.lines.push(String::new());

            // Limitar buffer
            if self.lines.len() > MAX_SCROLL_LINES {
                self.lines.remove(0);
                if self.scroll_offset > 0 {
                    self.scroll_offset -= 1;
                }
            }
        }
    }

    /// Backspace
    pub fn backspace(&mut self) {
        if self.cursor_x > 0 {
            self.cursor_x -= 1;
            let line_idx = self.scroll_offset + self.cursor_y as usize;
            if line_idx < self.lines.len() {
                let line = &mut self.lines[line_idx];
                if (self.cursor_x as usize) < line.len() {
                    line.remove(self.cursor_x as usize);
                }
            }
        }
    }

    /// Limpa o terminal
    pub fn clear(&mut self) {
        self.lines.clear();
        for _ in 0..self.rows {
            self.lines.push(String::new());
        }
        self.cursor_x = 0;
        self.cursor_y = 0;
        self.scroll_offset = 0;
    }

    /// Atualiza animação do cursor (chamado a cada frame)
    pub fn tick(&mut self) {
        self.cursor_blink_counter += 1;
        // Piscar a cada 30 frames (~500ms a 60fps)
        if self.cursor_blink_counter >= 30 {
            self.cursor_visible = !self.cursor_visible;
            self.cursor_blink_counter = 0;
        }
    }

    /// Retorna linha visível por índice (0 = topo)
    pub fn get_visible_line(&self, row: u32) -> Option<&str> {
        let idx = self.scroll_offset + row as usize;
        self.lines.get(idx).map(|s| s.as_str())
    }

    // TODO: Revisar no futuro
    #[allow(unused)]
    /// Scroll para cima
    pub fn scroll_up(&mut self, lines: usize) {
        if self.scroll_offset >= lines {
            self.scroll_offset -= lines;
        } else {
            self.scroll_offset = 0;
        }
    }

    // TODO: Revisar no futuro
    #[allow(unused)]
    /// Scroll para baixo
    pub fn scroll_down(&mut self, lines: usize) {
        let max_scroll = self.lines.len().saturating_sub(self.rows as usize);
        self.scroll_offset = (self.scroll_offset + lines).min(max_scroll);
    }
}

/// Converte um char para String (helper interno)
fn char_to_string(c: char) -> String {
    let mut s = String::new();
    s.push(c);
    s
}
