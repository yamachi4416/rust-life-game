mod help_widget;
mod life_game_widget;

use std::time::{Duration, Instant};

use ratatui::Frame;

pub use help_widget::{HelpWidget, HelpWidgetState};
pub use life_game_widget::{LifeGameWidget, LifeGameWidgetState};

pub struct AppState {
    pub life_game: LifeGameWidgetState,
    pub dialog: HelpWidgetState,
    pub last_tick: Instant,
    pub tick_rate: Duration,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            life_game: LifeGameWidgetState::new(),
            dialog: HelpWidgetState::new(),
            last_tick: Instant::now(),
            tick_rate: Duration::from_secs(1),
        }
    }

    pub fn reset_tick(&mut self) {
        self.last_tick = Instant::now();
    }

    pub fn trigger_step(&mut self) {
        if let Some(last_tick) = self.last_tick.checked_sub(self.tick_rate) {
            self.last_tick = last_tick;
        }
    }

    pub fn should_step(&self) -> bool {
        self.last_tick.elapsed() >= self.tick_rate
    }

    pub fn timeout(&self) -> Duration {
        self.tick_rate.saturating_sub(self.last_tick.elapsed())
    }
}

pub trait Draw {
    fn draw(&self, frame: &mut Frame, state: &AppState);
}
