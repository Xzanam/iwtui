mod ui;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    Frame, layout::{
        Constraint::{self, Length},
        Layout, Rows,
    }, style::{Color, Style}, text::{self, Line}, widgets::{Block, Borders, List, ListItem, ListState, Row, Table, TableState, Widget},
};
use std::io;
use ui::{
    layout::{Header, NetworkList},
    models::Networks,
};

use crate::ui::{layout::Footer, models::ConnectionStatus};

#[derive(Debug)]
struct App {
    available_networks: Vec<Networks>,
    table_state: TableState,
    should_quit: bool,
}

impl Default for App {
    fn default() -> Self {
        let available_networks = vec![
            Networks {
                id: 1,
                ssid: "ValarMorghulis".to_string(),
                security: "WPA2".to_string(),
            },
            Networks {
                id: 2,
                ssid: "ValarDohaeris".to_string(),
                security: "WPA/WPA2".to_string(),
            },
        ];

        let mut table_state = TableState::default();

        table_state.select_first();
        table_state.select_first_column();

        let should_quit = false;

        App {
            available_networks,
            table_state,
            should_quit,
        }
    }
}

impl App {
    fn run(&mut self, terminal: &mut ratatui::DefaultTerminal) -> Result<()> {
        while !self.should_quit {
            terminal.draw(|frame| self.render(frame))?;

            if let Some(key) = event::read()?.as_key_press_event() {
                match key.code {
                    KeyCode::Char('q') => self.should_quit = true,
                    KeyCode::Char('j') => self.table_state.select_next(),
                    KeyCode::Char('k') => self.table_state.select_previous(),
                    _ => (),
                }
            }
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let areas = Layout::vertical([
            Constraint::Length(4),
            Constraint::Fill(5),
            Constraint::Length(4),
        ])
        .spacing(1)
        .split(frame.area());

        let header = Header::new("wls20", ConnectionStatus::Connected);

        let rows : Vec<Row> = self
            .available_networks
            .iter()
            .map(|item| Row::new([item.ssid.as_str(), item.security.as_str()]))
            .collect();

        let table_cols = Row::new(["SSID", "Security"])
            .style(Style::new().bold())
            .bottom_margin(1);

        let widths = [Constraint::Percentage(30), Constraint::Percentage(20)];

        let table = Table::new(rows, widths)
            .block(Block::default().borders(Borders::ALL).title(Line::from("Available Networks").centered()))
            .header(table_cols)
            .column_spacing(1)
            .style(Color::White)
            .row_highlight_style(Style::new().on_black().bold())
            .cell_highlight_style(Style::new().reversed().red());

        frame.render_widget(header, areas[0]);
        frame.render_stateful_widget(table, areas[1], &mut self.table_state);
        frame.render_widget(
            Footer {
                help_text: String::from("Press q to quit."),
            },
            areas[2],
        );
    }

    fn handle_input(&self, event: Event) {
        todo!();
    }
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut app = App::default();

    ratatui::run(|terminal| app.run(terminal));

    Ok(())
}
