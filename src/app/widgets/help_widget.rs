use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Cell, Clear, Row, Table},
    Frame,
};

use super::{AppState, Draw};

pub struct HelpWidgetState {
    pub is_open: bool,
    pub title: String,
    pub items: Vec<(String, String)>,
}

impl HelpWidgetState {
    pub fn new() -> Self {
        Self {
            is_open: false,
            title: String::new(),
            items: Vec::new(),
        }
    }

    pub fn open(
        &mut self,
        title: impl Into<String>,
        items: Vec<(impl Into<String>, impl Into<String>)>,
    ) {
        self.title = title.into();
        self.items = items
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect();
        self.is_open = true;
    }

    pub fn close(&mut self) {
        self.is_open = false;
    }
}

pub struct HelpWidget;

impl HelpWidget {
    pub fn new() -> Self {
        Self
    }
}

impl Draw for HelpWidget {
    fn draw(&self, frame: &mut Frame, state: &AppState) {
        if !state.dialog.is_open {
            return;
        }

        let area = frame.area();

        let (max_key, max_desc) = state.dialog.items.iter().fold((0, 0), |(km, dm), (k, d)| {
            (km.max(k.len()), dm.max(d.len()))
        });

        let widths = [
            Constraint::Length((max_key + 1) as u16),
            Constraint::Min((max_desc + 2) as u16),
        ];

        let popup_width =
            ((max_key + max_desc + 7).max(state.dialog.title.len() + 4) as u16).min(area.width);
        let popup_height = ((state.dialog.items.len() as u16) + 2).min(area.height);

        let popup_area = Rect {
            x: area.x + (area.width.saturating_sub(popup_width)) / 2,
            y: area.y + (area.height.saturating_sub(popup_height)) / 2,
            width: popup_width,
            height: popup_height,
        };

        frame.render_widget(Clear, popup_area);
        frame.render_widget(
            Table::new(
                state.dialog.items.iter().map(|(key, desc)| {
                    Row::new(vec![
                        Cell::from(format!(" {key}")).style(
                            Style::default()
                                .fg(Color::White)
                                .add_modifier(Modifier::BOLD),
                        ),
                        Cell::from(format!("| {desc}")).style(Style::default().fg(Color::White)),
                    ])
                }),
                widths,
            )
            .block(
                Block::default()
                    .title(format!(" {} ", state.dialog.title))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Plain)
                    .style(Style::default()),
            ),
            popup_area,
        );
    }
}
