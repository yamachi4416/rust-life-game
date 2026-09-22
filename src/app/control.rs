use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

use crate::app::widgets::AppState;

pub enum AppAction {
    Quit,
    Next,
    None,
}

pub struct AppController;

impl AppController {
    pub fn new() -> Self {
        Self
    }

    pub fn handle_key_event(&self, state: &mut AppState, key: KeyEvent) -> AppAction {
        if key.kind == KeyEventKind::Release {
            return AppAction::None;
        }

        match key.code {
            KeyCode::Char('q') => AppAction::Quit,
            KeyCode::Char('n') => {
                state.reset_tick();
                AppAction::Next
            }
            KeyCode::Char(' ') => {
                state.trigger_step();
                AppAction::None
            }
            KeyCode::Char('+') => {
                state.life_game.add_size(1);
                AppAction::None
            }
            KeyCode::Char('-') => {
                state.life_game.add_size(-1);
                AppAction::None
            }
            KeyCode::Char('c') => {
                state.life_game.next_color();
                AppAction::None
            }
            KeyCode::Right | KeyCode::Char('l') => {
                state.life_game.move_x(1);
                AppAction::None
            }
            KeyCode::Left | KeyCode::Char('h') => {
                state.life_game.move_x(-1);
                AppAction::None
            }
            KeyCode::Down | KeyCode::Char('j') => {
                state.life_game.move_y(1);
                AppAction::None
            }
            KeyCode::Up | KeyCode::Char('k') => {
                state.life_game.move_y(-1);
                AppAction::None
            }
            _ => AppAction::None,
        }
    }
}
