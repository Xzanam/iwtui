mod ui;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    Frame, layout::{
        Constraint::{self, Length}, Layout, Margin, Rows,
    }, style::{Color, Style}, text::{self, Line}, widgets::{Block, Borders, Cell, List, ListItem, ListState, Padding, Row, Table, TableState, Widget},
};
use std::io;
use ui::{
    layout::{Header, NetworkList},
    models::Network,
};

use crate::ui::{layout::Footer, models::{ConnectionStatus, WifiBand, wifisignal::SignalStrength}};

#[derive(Debug)]
struct App {
    available_networks: Vec<Network>,
    table_state: TableState,
    should_quit: bool,
}

impl Default for App {
    fn default() -> Self {
        let available_networks = get_dummy_networks();

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
                    KeyCode::Char('j') | KeyCode::Down => self.table_state.select_next(),
                    KeyCode::Char('k') | KeyCode::Up => self.table_state.select_previous(),
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
        .split(frame.area().inner(Margin { vertical: 1, horizontal: 2}));

        let header = Header::new("wls20", ConnectionStatus::Connected);

        let rows: Vec<Row> = self
            .available_networks
            .iter()
            .map(|item| {
                Row::new([
                    Cell::from(item.ssid.clone()),
                    Cell::from(item.security.clone()),
                    Cell::from(item.band.to_string()),
                    item.strength.into(), 
                ]).bottom_margin(1)
            })
            .collect();

        let table_header = Row::new(["SSID", "Security", "Band", "Strength"])
            .style(Style::new().bold())
            .bottom_margin(1);

        let widths = [Constraint::Percentage(20), Constraint::Percentage(10),Constraint::Percentage(10), Constraint::Percentage(10) ];

        let table = Table::new(rows, widths)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(Line::from("Available Networks").centered())
                    .padding(Padding::symmetric(2, 1)),
            )
            .header(table_header)
            .column_spacing(1)
            .style(Color::White)
            .row_highlight_style(Style::new().on_black().bold())
            .cell_highlight_style(Style::new().reversed().red());


        
        

        frame.render_widget(header, areas[0]);
        frame.render_stateful_widget(table,areas[1], &mut self.table_state);
        frame.render_widget(
            Footer {
                help_text: String::from(
                    "Press q to quit |  j k or ↑ ↓ to navigate |  Enter to select",
                ),
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

fn get_dummy_networks() -> Vec<Network> {
    let mut networks = Vec::new();

    networks.push(Network {
        id: 1,
        ssid: String::from("mayac@"),
        security: String::from("WPA2"),
        band : WifiBand::B24, 
        strength : SignalStrength(23), 
    });

    networks.push(Network {
        id: 1,
        ssid: String::from("ValarMorgulis"),
        security: String::from("WPA2"),
        band : WifiBand::B24, 
        strength : SignalStrength(100)

    });

    networks.push(Network {
        id: 1,
        ssid: String::from("SegmentationFault"),
        security: String::from("WPA2"),
        band : WifiBand::B5, 
        strength : SignalStrength(56)
    });

    networks.push(Network {
        id: 1,
        ssid: String::from("HiddenNetwork"),
        security: String::from("WPA2"),
        band : WifiBand::B5, 
        strength : SignalStrength(12)
    });

    networks
}
