use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::Text,
    widgets::Block,
    Frame,
};
use rust_life_game::LifeGame;

use super::{AppState, Draw};

pub struct LifeGameWidgetState {
    pub x: u16,
    pub y: u16,
    pub size: u16,
    pub color: u8,
    pub life_game: LifeGame,
}

impl LifeGameWidgetState {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            size: 1,
            color: 0,
            life_game: LifeGame::new(0, 0),
        }
    }

    pub fn add_size(&mut self, delta: i16) {
        if let Some(delta) = self.size.checked_add_signed(delta) {
            self.size = delta.clamp(1, 10);
        }
    }

    pub fn move_x(&mut self, x: i16) {
        if let Some(x) = self.x.checked_add_signed(x) {
            self.x = x.clamp(0, 100);
        }
    }

    pub fn move_y(&mut self, y: i16) {
        if let Some(y) = self.y.checked_add_signed(y) {
            self.y = y.clamp(0, 100);
        }
    }

    pub fn next_color(&mut self) {
        self.color = (self.color + 1) % 16;
    }
}

pub struct LifeGameWidget;

impl LifeGameWidget {
    pub fn new() -> Self {
        Self
    }
}

impl Draw for LifeGameWidget {
    fn draw(&self, frame: &mut Frame, state: &AppState) {
        let widget_state = &state.life_game;
        let color = Color::Indexed(widget_state.color);

        let style_title = Style::default().bg(color).bold();
        let style_live = Style::default().bg(color);
        let style_dead = Style::default().bg(Color::White);

        let title = Text::from(widget_state.life_game.name()).style(style_title);
        let title_height = title.height() as u16;

        let width = widget_state.size * 2;
        let height = widget_state.size;

        frame.render_widget(
            title.centered(),
            Rect {
                x: widget_state.x,
                y: widget_state.y,
                width: widget_state.life_game.width() * width,
                height: title_height,
            },
        );

        for (y, rows) in widget_state.life_game.cells_iter().enumerate() {
            let y = y as u16 * height + title_height + widget_state.y;

            for (x, col) in rows.enumerate() {
                frame.render_widget(
                    Block::default().style(if col { style_live } else { style_dead }),
                    Rect {
                        x: x as u16 * width + widget_state.x,
                        y,
                        height,
                        width,
                    },
                );
            }
        }
    }
}
