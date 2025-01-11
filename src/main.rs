use std::{
    error::Error,
    time::{Duration, Instant},
};

use crossterm::event::{self, Event, KeyCode, KeyEvent};

use ratatui::{
    layout::Rect,
    style::{Color, Style, Stylize},
    text::Text,
    widgets::Block,
    DefaultTerminal, Frame,
};
use rust_life_game::LifeGame;

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = ratatui::init();
    let mut app = App::new();
    let result = app.run(&mut terminal);
    ratatui::restore();
    result
}

struct Setting {
    x: u16,
    y: u16,
    size: u16,
    color: u8,
    tick_rate: Duration,
}

impl Setting {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            size: 1,
            color: 0,
            tick_rate: Duration::from_secs(1),
        }
    }

    fn add_size(&mut self, delta: i16) {
        self.size = (self.size as i32 + delta as i32).clamp(1, 10) as u16;
    }

    fn next_color(&mut self) {
        self.color = (self.color + 1) % 16;
    }

    fn move_x(&mut self, x: i16) {
        self.x = (self.x as i32 + x as i32).clamp(0, 100) as u16;
    }

    fn move_y(&mut self, y: i16) {
        self.y = (self.y as i32 + y as i32).clamp(0, 100) as u16;
    }
}

enum HandleResult {
    Quit,
    Next,
    Keep,
}

struct App {
    setting: Setting,
    last_tick: Instant,
    input_name: String,
    life_game: LifeGame,
}

impl App {
    fn new() -> Self {
        App {
            setting: Setting::new(),
            last_tick: Instant::now(),
            input_name: "".into(),
            life_game: LifeGame::new(0, 0),
        }
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<(), Box<dyn Error>> {
        self.last_tick = Instant::now();

        for (name, input) in inputs().iter().cycle() {
            self.input_name = name.into();
            self.life_game = LifeGame::from(input);

            loop {
                terminal.draw(|frame| self.draw(frame))?;

                if event::poll(
                    self.setting
                        .tick_rate
                        .saturating_sub(self.last_tick.elapsed()),
                )? {
                    if let Event::Key(key) = event::read()? {
                        match self.handle_key_event(key) {
                            HandleResult::Quit => return Ok(()),
                            HandleResult::Next => break,
                            HandleResult::Keep => {}
                        }
                    }
                }

                if self.last_tick.elapsed() < self.setting.tick_rate {
                    continue;
                }

                self.last_tick = Instant::now();

                if let None = self.life_game.next() {
                    break;
                }
            }
        }

        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> HandleResult {
        match key.code {
            KeyCode::Char('q') => return HandleResult::Quit,
            KeyCode::Char('n') => {
                self.last_tick = Instant::now();
                return HandleResult::Next;
            }
            KeyCode::Char('+') => self.setting.add_size(1),
            KeyCode::Char('-') => self.setting.add_size(-1),
            KeyCode::Char('c') => self.setting.next_color(),
            KeyCode::Right | KeyCode::Char('l') => self.setting.move_x(1),
            KeyCode::Left | KeyCode::Char('h') => self.setting.move_x(-1),
            KeyCode::Down | KeyCode::Char('j') => self.setting.move_y(1),
            KeyCode::Up | KeyCode::Char('k') => self.setting.move_y(-1),
            KeyCode::Char(' ') => {
                self.last_tick = self.last_tick.checked_sub(self.setting.tick_rate).unwrap();
            }
            _ => {}
        }
        HandleResult::Keep
    }

    fn draw(&self, frame: &mut Frame) {
        let name = &self.input_name;
        let game = &self.life_game;

        let color = Color::Indexed(self.setting.color);

        let style_title = Style::default().bg(color).bold();
        let style_live = Style::default().bg(color);
        let style_dead = Style::default().bg(Color::White);

        let title = Text::from_iter([name.as_ref()]).style(style_title);
        let title_height = title.height() as u16;

        let width = self.setting.size * 2;
        let height = self.setting.size;

        frame.render_widget(
            title.centered(),
            Rect {
                x: self.setting.x,
                y: self.setting.y,
                width: game.width as u16 * width,
                height: title_height,
            },
        );

        for (y, rows) in game.cells_iter().enumerate() {
            let y = y as u16 * height + title_height + self.setting.y;

            for (x, col) in rows.enumerate() {
                frame.render_widget(
                    Block::default().style(if col { style_live } else { style_dead }),
                    Rect {
                        x: x as u16 * width + self.setting.x,
                        y,
                        height,
                        width,
                    },
                );
            }
        }
    }
}

fn inputs() -> Vec<(String, Vec<Vec<u8>>)> {
    vec![
        (
            "octagon".to_uppercase(),
            vec![
                vec![0, 0, 0, 1, 1, 0, 0, 0],
                vec![0, 0, 1, 0, 0, 1, 0, 0],
                vec![0, 1, 0, 0, 0, 0, 1, 0],
                vec![1, 0, 0, 0, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 0, 0, 0, 1],
                vec![0, 1, 0, 0, 0, 0, 1, 0],
                vec![0, 0, 1, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 1, 1, 0, 0, 0],
            ],
        ),
        (
            "glider".to_uppercase(),
            vec![
                vec![0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "twin-glider".to_uppercase(),
            vec![
                vec![0, 0, 1, 0, 0, 0, 0, 0, 0, 0],
                vec![1, 0, 1, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 1, 0, 0, 0, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 1, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "galaxy".to_uppercase(),
            vec![
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            ],
        ),
        (
            "tree".to_uppercase(),
            vec![
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
                vec![0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            ],
        ),
    ]
}
