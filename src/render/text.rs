//! # Text Rendering
//!
//! Renderização de texto usando fonte bitmap.

use super::colors;
use super::font::{BitFont, CHAR_HEIGHT, CHAR_WIDTH};
use gfx_types::color::Color;
use redpowder::window::Window;

/// Renderizador de texto
pub struct TextRenderer {
    /// Fonte utilizada
    font: BitFont,
    /// Cor do texto
    pub fg_color: u32,
    /// Cor de fundo
    pub bg_color: u32,
}

impl Default for TextRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl TextRenderer {
    /// Cria novo renderizador
    pub const fn new() -> Self {
        Self {
            font: BitFont::new(),
            fg_color: colors::TEXT,
            bg_color: colors::BACKGROUND,
        }
    }

    /// Desenha um caractere na posição especificada
    pub fn draw_char(&self, window: &mut Window, x: u32, y: u32, c: char) {
        let glyph = self.font.get_glyph(c);

        for (row, &byte) in glyph.iter().enumerate() {
            for col in 0..8 {
                let pixel_x = x + col;
                let pixel_y = y + row as u32;

                // Verificar se o bit está ativo
                let bit = (byte >> (7 - col)) & 1;
                let color = if bit == 1 {
                    self.fg_color
                } else {
                    self.bg_color
                };

                window.put_pixel(pixel_x, pixel_y, Color(color));
            }
        }
    }

    /// Desenha uma string na posição especificada
    pub fn draw_string(&self, window: &mut Window, x: u32, y: u32, text: &str) {
        let mut cursor_x = x;

        for c in text.chars() {
            if c == '\n' {
                continue; // Ignorar newlines (tratados externamente)
            }

            self.draw_char(window, cursor_x, y, c);
            cursor_x += CHAR_WIDTH;
        }
    }

    /// Desenha uma string com cor específica
    pub fn draw_string_colored(
        &self,
        window: &mut Window,
        x: u32,
        y: u32,
        text: &str,
        fg: u32,
        bg: u32,
    ) {
        let mut cursor_x = x;

        for c in text.chars() {
            if c == '\n' {
                continue;
            }

            self.draw_char_colored(window, cursor_x, y, c, fg, bg);
            cursor_x += CHAR_WIDTH;
        }
    }

    /// Desenha caractere com cores específicas
    fn draw_char_colored(&self, window: &mut Window, x: u32, y: u32, c: char, fg: u32, bg: u32) {
        let glyph = self.font.get_glyph(c);

        for (row, &byte) in glyph.iter().enumerate() {
            for col in 0..8 {
                let pixel_x = x + col;
                let pixel_y = y + row as u32;

                let bit = (byte >> (7 - col)) & 1;
                let color = if bit == 1 { fg } else { bg };

                window.put_pixel(pixel_x, pixel_y, Color(color));
            }
        }
    }

    /// Retorna largura do caractere
    pub const fn char_width(&self) -> u32 {
        CHAR_WIDTH
    }

    /// Retorna altura do caractere
    pub const fn char_height(&self) -> u32 {
        CHAR_HEIGHT
    }
}
