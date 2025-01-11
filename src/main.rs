use std::{error::Error, time::Duration};

use crossterm::event;

use ratatui::{
    layout,
    style::Color,
    widgets::{
        canvas::{Canvas, Points},
        Block, BorderType,
    },
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
    let area = layout::Rect::new(0, 0, game.width as u16, game.height as u16);

    let canvas = Canvas::default()
        .block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title(name)
                .title_alignment(layout::Alignment::Center),
        )
        .marker(ratatui::symbols::Marker::Dot)
        .x_bounds([0.0, f64::from(area.width)])
        .y_bounds([0.0, f64::from(area.height)])
        .paint(|ctx| {
            ctx.draw(&Points {
                coords: &game
                    .cells_iter()
                    .enumerate()
                    .flat_map(|(y, rows)| {
                        rows.enumerate().filter(|(_, c)| *c).map(move |(x, _)| {
                            (
                                x as f64 - f64::from(area.left()) + 1.0,
                                f64::from(area.bottom()) - y as f64 - 1.0,
                            )
                        })
                    })
                    .collect::<Vec<_>>(),
                color: Color::White,
            });
        });

    frame.render_widget(canvas, area);
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
