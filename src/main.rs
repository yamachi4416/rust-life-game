use std::{error::Error, time::Duration};

use crossterm::event;

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
    let result = run(&mut terminal);
    ratatui::restore();
    result
}

fn run(terminal: &mut DefaultTerminal) -> Result<(), Box<dyn Error>> {
    let inputs = inputs();

    loop {
        for (name, input) in inputs.iter() {
            let mut game = LifeGame::from(input);

            loop {
                terminal.draw(|frame| draw(&name, &game, frame))?;

                if event::poll(Duration::from_millis(1000))? {
                    if let event::Event::Key(key) = event::read()? {
                        match key.code {
                            event::KeyCode::Enter => {
                                break;
                            }
                            event::KeyCode::Char('q') => {
                                return Ok(());
                            }
                            _ => {}
                        }
                    }
                }

                if let None = game.next() {
                    break;
                }
            }
        }
    }
}

fn draw(name: &str, game: &LifeGame, frame: &mut Frame) {
    let title = Text::from_iter([name]).bg(Color::Blue).fg(Color::White);
    let title_height = title.height() as u16;

    let width = 2;
    let height = 1;

    let style_live = Style::default().bg(Color::LightBlue);
    let style_dead = Style::default().bg(Color::White);

    frame.render_widget(
        title.centered(),
        Rect {
            x: 0,
            y: 0,
            width: game.width as u16 * width,
            height: title_height,
        },
    );

    for (y, rows) in game.cells_iter().enumerate() {
        for (x, col) in rows.enumerate() {
            frame.render_widget(
                Block::default().style(if col { style_live } else { style_dead }),
                Rect {
                    x: x as u16 * width,
                    y: y as u16 * height + title_height,
                    height,
                    width,
                },
            );
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
