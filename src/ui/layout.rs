use ratatui::{
    buffer::Buffer, layout::Rect, style::{Color, Style}, text::Line, widgets::{Block, Borders, List, ListItem, ListState, Paragraph, StatefulWidget, Widget},
};

use super::models::{ConnectionStatus, Networks};

#[derive(Default, Debug)]
pub struct Header {
    pub interface: String,
    pub connection_status: ConnectionStatus,
}

impl Header { 
    pub fn new(interface : &str, connection_status: ConnectionStatus) -> Self { 
        Header { 
            interface : interface.to_string(), 
            connection_status
        }
    }
    
}

impl Widget for Header {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text = format!(
            " Interface: {} | Status: {:?}",
            self.interface, self.connection_status
        );
        Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title(Line::from("IWTUI").centered()))
            .render(area, buf);
    }
}

#[derive(Default, Debug)]
pub struct NetworkList {
    state: ListState,
    networks: Vec<Networks>,
}

#[derive(Default, Debug)]
pub struct Footer {
    pub help_text: String,
}

impl Widget for Footer {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(self.help_text)
            .block(Block::default().borders(Borders::ALL).title(Line::from("Navigation").centered()))
            .render(area, buf);
    }
}
