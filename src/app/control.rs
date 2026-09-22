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

        if state.dialog.is_open {
            match key.code {
                KeyCode::Char('?') | KeyCode::Char('q') | KeyCode::Esc => {
                    state.dialog.close();
                }
                _ => {}
            }
            return AppAction::None;
        }

        match key.code {
            KeyCode::Char('q') => AppAction::Quit,
            KeyCode::Char('?') => {
                self.open_help(state);
                AppAction::None
            }
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

    fn open_help(&self, state: &mut AppState) {
        state.dialog.open(
            "Help (? / q / Esc: Close)",
            vec![
                ("h, j, k, l / Arrow", "Move view"),
                ("+ / -", "Zoom size"),
                ("c", "Change color"),
                ("Space", "Step forward"),
                ("n", "Next pattern"),
                ("? / Esc", "Toggle / Close"),
                ("q", "Quit"),
            ],
        );
    }
}
