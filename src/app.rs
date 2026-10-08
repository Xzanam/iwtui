use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use nmrs::Network;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Margin, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Cell, Padding, Row, Table, TableState},
};
use tokio::sync::mpsc;

use crate::{
    networkmanager::{
        backend::NetworkBackend,
        message::{NetworkCommand, NetworkEvent},
    },
    ui::{
        layout::{Footer, Header},
        models::{ConnectionStatus, WifiBand, WifiNetwork, wifisignal::SignalStrength},
    },
};

#[derive(Debug)]
pub struct App {
    command_tx: mpsc::Sender<NetworkCommand>,
    event_rx: mpsc::Receiver<NetworkEvent>,
    available_networks: Vec<WifiNetwork>,
    table_state: TableState,
    should_quit: bool,
}

impl App {
    pub fn new(
        command_tx: mpsc::Sender<NetworkCommand>,
        event_rx: mpsc::Receiver<NetworkEvent>,
    ) -> Self {
        let available_networks = Vec::new();
        let mut table_state = TableState::default();
        table_state.select_first();
        table_state.select_first_column();

        command_tx.try_send(NetworkCommand::Scan);

        let should_quit = false;

        App {
            command_tx,
            event_rx,
            available_networks,
            table_state,
            should_quit,
        }
    }
    pub fn run(&mut self, terminal: &mut ratatui::DefaultTerminal) -> Result<()> {
        while !self.should_quit {
            terminal.draw(|frame| self.render(frame))?;

            self.process_network_events();

            if event::poll(Duration::from_millis(50))? {
                if let Some(key) = event::read()?.as_key_press_event() {
                    match key.code {
                        KeyCode::Char('q') => self.should_quit = true,
                        KeyCode::Char('j') | KeyCode::Down => self.table_state.select_next(),
                        KeyCode::Char('k') | KeyCode::Up => self.table_state.select_previous(),
                        KeyCode::Char('s') => self.scan(),
                        KeyCode::Char('p') =>  { 
                            println!("Networks : {:?} ", self.available_networks);
                        }
                        KeyCode::Enter => self.try_connect(),
                        _ => (),
                    }
                }
            }
        }
        Ok(())
    }

    fn process_network_events(&mut self) {
        while let Ok(event) = self.event_rx.try_recv() {
            self.handle_network_event(event);
        }
    }

    fn handle_network_event(&mut self, event: NetworkEvent) {
        match event {
            NetworkEvent::ScanCompleted(networks) => {
                self.available_networks = networks;
            }
            _ => {
            }
        }
    }

    fn try_connect(&mut self) {
        println!("trying  to connect");
    }

    fn render(&mut self, frame: &mut Frame) {
        let areas = Layout::vertical([
            Constraint::Length(4),
            Constraint::Fill(5),
            Constraint::Length(4),
        ])
        .split(frame.area().inner(Margin {
            vertical: 1,
            horizontal: 2,
        }));

        self.render_header(frame, areas[0]);
        self.render_table(frame, areas[1]);
        self.render_footer(frame, areas[2]);
    }

    fn handle_input(&self, event: Event) {
        todo!();
    }

    fn render_header(&mut self, frame: &mut Frame, area: Rect) {
        let header = Header::new("wls20", ConnectionStatus::Connected);
        frame.render_widget(header, area);
    }
    fn render_table(&mut self, frame: &mut Frame, area: Rect) {
        let rows: Vec<Row> = self
            .available_networks
            .iter()
            .map(|item| {
                Row::new([
                    Cell::from(item.ssid.clone()),
                    Cell::from(item.security.clone()),
                    Cell::from(item.band.to_string()),
                    item.strength.into(),
                ])
                .bottom_margin(1)
            })
            .collect();

        let table_header = Row::new(["SSID", "Security", "Band", "Strength"])
            .style(Style::new().bold())
            .bottom_margin(1);

        let widths = [
            Constraint::Percentage(20),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
        ];

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

        frame.render_stateful_widget(table, area, &mut self.table_state);
    }
    fn render_footer(&mut self, frame: &mut Frame, area: Rect) {
        frame.render_widget(
            Footer {
                help_text: String::from(
                    "Press q to quit |  j k or ↑ ↓ to navigate |  Enter to select",
                ),
            },
            area,
        );
    }

    fn scan(&self) {
        self.command_tx.try_send(NetworkCommand::Scan);
    }
}
/* fn get_dummy_networks() -> Vec<Network> {
    let mut networks = Vec::new();

    networks.push(Network {
        id: 1,
        ssid: String::from("mayac@"),
        security: String::from("WPA2"),
        band: WifiBand::B24,
        strength: SignalStrength(23),
    });

    networks.push(Network {
        id: 1,
        ssid: String::from("ValarMorgulis"),
        security: String::from("WPA2"),
        band: WifiBand::B24,
        strength: SignalStrength(100),
    });

    networks.push(Network {
        id: 1,
        ssid: String::from("SegmentationFault"),
        security: String::from("WPA2"),
        band: WifiBand::B5,
        strength: SignalStrength(56),
    });

    networks.push(Network {
        id: 1,
        ssid: String::from("HiddenNetwork"),
        security: String::from("WPA2"),
        band: WifiBand::B5,
        strength: SignalStrength(12),
    });

    networks
}
 */
