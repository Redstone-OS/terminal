//! # Window Decorations
//!
//! Decorações de janela (barra de título, botões, borda).

use crate::render::colors;
use crate::render::font::{CHAR_HEIGHT, CHAR_WIDTH};
use crate::render::text::TextRenderer;
use gfx_types::color::Color;
use gfx_types::geometry::Rect;
use redpowder::window::Window;

/// Altura da barra de título
pub const TITLE_BAR_HEIGHT: u32 = 28;

/// Largura dos botões da barra de título
pub const BUTTON_WIDTH: u32 = 46;

/// Largura da borda da janela
pub const BORDER_WIDTH: u32 = 1;

/// Padding interno da área de conteúdo
pub const CONTENT_PADDING: u32 = 4;

/// Decorações de janela
pub struct WindowDecorations {
    /// Título da janela
    pub title: &'static str,
    /// Largura total da janela
    pub width: u32,
    /// Altura total da janela
    pub height: u32,
    /// Janela está ativa/focada
    pub is_active: bool,
}

impl WindowDecorations {
    /// Cria decorações para janela
    pub fn new(title: &'static str, width: u32, height: u32) -> Self {
        Self {
            title,
            width,
            height,
            is_active: true,
        }
    }

    /// Desenha as decorações na janela
    pub fn draw(&self, window: &mut Window) {
        // Barra de título
        self.draw_title_bar(window);

        // Borda
        self.draw_border(window);
    }

    /// Desenha a barra de título
    fn draw_title_bar(&self, window: &mut Window) {
        let bg_color = if self.is_active {
            colors::TITLE_BAR_BG_ACTIVE
        } else {
            colors::TITLE_BAR_BG
        };

        // Fundo da barra de título
        let title_rect = Rect::new(0, 0, self.width, TITLE_BAR_HEIGHT);
        window.fill_rect(title_rect, Color(bg_color));

        // Título centralizado
        let renderer = TextRenderer::new();
        let title_width = self.title.len() as u32 * CHAR_WIDTH;
        let title_x = (self.width - title_width) / 2;
        let title_y = (TITLE_BAR_HEIGHT - CHAR_HEIGHT) / 2;

        renderer.draw_string_colored(
            window,
            title_x,
            title_y,
            self.title,
            colors::TITLE_TEXT,
            bg_color,
        );

        // Botão de fechar (X)
        self.draw_close_button(window, bg_color);

        // Botão de minimizar (-)
        self.draw_minimize_button(window, bg_color);
    }

    /// Desenha botão de fechar
    fn draw_close_button(&self, window: &mut Window, _bg: u32) {
        let btn_x = self.width - BUTTON_WIDTH;
        let btn_y = 0;

        // Fundo do botão (cor de hover simulada para visibilidade)
        let btn_rect = Rect::new(btn_x as i32, btn_y, BUTTON_WIDTH, TITLE_BAR_HEIGHT);
        window.fill_rect(btn_rect, Color(colors::CLOSE_BUTTON_HOVER));

        // X no centro
        let x_char_x = btn_x + (BUTTON_WIDTH - CHAR_WIDTH) / 2;
        let x_char_y = (TITLE_BAR_HEIGHT - CHAR_HEIGHT) / 2;

        let renderer = TextRenderer::new();
        renderer.draw_string_colored(
            window,
            x_char_x,
            x_char_y,
            "X",
            colors::TITLE_TEXT,
            colors::CLOSE_BUTTON_HOVER,
        );
    }

    /// Desenha botão de minimizar
    fn draw_minimize_button(&self, window: &mut Window, _bg: u32) {
        let btn_x = self.width - BUTTON_WIDTH * 2;
        let btn_y = 0;

        // Fundo do botão
        let btn_rect = Rect::new(btn_x as i32, btn_y as i32, BUTTON_WIDTH, TITLE_BAR_HEIGHT);
        window.fill_rect(btn_rect, Color(colors::MINIMIZE_BUTTON_HOVER));

        // - no centro
        let char_x = btn_x + (BUTTON_WIDTH - CHAR_WIDTH) / 2;
        let char_y = (TITLE_BAR_HEIGHT - CHAR_HEIGHT) / 2;

        let renderer = TextRenderer::new();
        renderer.draw_string_colored(
            window,
            char_x,
            char_y,
            "-",
            colors::TITLE_TEXT,
            colors::MINIMIZE_BUTTON_HOVER,
        );
    }

    /// Desenha borda da janela
    fn draw_border(&self, window: &mut Window) {
        let color = Color(colors::WINDOW_BORDER);

        // Esquerda
        let left_rect = Rect::new(
            0,
            TITLE_BAR_HEIGHT as i32,
            BORDER_WIDTH,
            self.height - TITLE_BAR_HEIGHT,
        );
        window.fill_rect(left_rect, color);

        // Direita
        let right_rect = Rect::new(
            (self.width - BORDER_WIDTH) as i32,
            TITLE_BAR_HEIGHT as i32,
            BORDER_WIDTH,
            self.height - TITLE_BAR_HEIGHT,
        );
        window.fill_rect(right_rect, color);

        // Inferior
        let bottom_rect = Rect::new(
            0,
            (self.height - BORDER_WIDTH) as i32,
            self.width,
            BORDER_WIDTH,
        );
        window.fill_rect(bottom_rect, color);
    }

    /// Retorna a área de conteúdo disponível (x, y, width, height)
    pub fn content_area(&self) -> (u32, u32, u32, u32) {
        let x = BORDER_WIDTH + CONTENT_PADDING;
        let y = TITLE_BAR_HEIGHT + CONTENT_PADDING;
        let w = self.width - (BORDER_WIDTH * 2) - (CONTENT_PADDING * 2);
        let h = self.height - TITLE_BAR_HEIGHT - BORDER_WIDTH - (CONTENT_PADDING * 2);
        (x, y, w, h)
    }
}
