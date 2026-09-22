mod control;
mod widgets;

use std::error::Error;

use crossterm::event::{self, Event};
use ratatui::{DefaultTerminal, Frame};
use rust_life_game::LifeGame;

use self::control::{AppAction, AppController};
use self::widgets::{AppState, Draw, LifeGameWidget};

pub struct App<'a> {
    state: AppState,
    controller: AppController,
    inputs: &'a Vec<(String, Vec<Vec<u8>>)>,
    widgets: Vec<Box<dyn Draw>>,
}

impl<'a> App<'a> {
    pub fn new(inputs: &'a Vec<(String, Vec<Vec<u8>>)>) -> Self {
        App {
            state: AppState::new(),
            controller: AppController::new(),
            inputs,
            widgets: vec![Box::new(LifeGameWidget::new())],
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<(), Box<dyn Error>> {
        self.state.reset_tick();

        for (name, input) in self.inputs.iter().cycle() {
            self.state.life_game.life_game = LifeGame::from(name, input);

            loop {
                terminal.draw(|frame| self.draw(frame))?;

                if event::poll(self.state.timeout())? {
                    if let Event::Key(key) = event::read()? {
                        match self.controller.handle_key_event(&mut self.state, key) {
                            AppAction::Quit => return Ok(()),
                            AppAction::Next => break,
                            AppAction::None => {}
                        }
                    }
                }

                if self.state.should_step() {
                    self.state.reset_tick();
                    if self.state.life_game.life_game.step().is_none() {
                        break;
                    }
                }
            }
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        for widget in &self.widgets {
            widget.draw(frame, &self.state);
        }
    }
}
