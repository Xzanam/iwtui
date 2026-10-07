use ratatui::{
    style::{Color, Modifier, Style}, text::{Line, Span}, widgets::Cell,
};

/// 1. DOMAIN MODEL: Type-safe signal wrapper
#[derive(Default, Debug, Clone, Copy)]
pub struct SignalStrength(pub u8); // Holds 0 to 100

impl SignalStrength {
    /// Maps the 0-100 percentage down to a 0-4 bar scale
    pub fn to_bars(&self) -> usize {
        match self.0 {
            0..=20   => 0,
            21..=40  => 1,
            41..=60  => 2,
            61..=80  => 3,
            81..=100 => 4,
            _ => 5,  // Just added for non exhaustive issue
        }
    }

    /// Determines the semantic color of the connection health
    pub fn to_color(&self) -> Color {
        match self.0 {
            0..=30  => Color::Red,    // Poor connection
            31..=65 => Color::Yellow, // Moderate connection
            _       => Color::Green,  // Excellent connection
        }
    }
}

impl SignalStrength {
    /// Renders a classic step-up signal indicator with colored active bars 
    /// and dimmed inactive bars.
    pub fn to_table_cell(&self) -> Cell<'static> {
        let active_count = match self.0 {
            0..=10  => 0, // No signal
            11..=35 => 1, // Minimal
            36..=60 => 2, // Low
            61..=85 => 3, // Medium
            _       => 4, // Excellent
        };

        // All 4 possible height steps (increasing size)
        let characters = ["▂", "▄", "▆", "█"];
        let active_color = self.to_color(); // Red/Yellow/Green logic from before

        let mut spans = Vec::new();

        for (i, &ch) in characters.iter().enumerate() {
            if i < active_count {
                // Active bar gets the bright connection color
                spans.push(Span::styled(ch, Style::default().fg(active_color)));
            } else {
                // Inactive bar gets dimmed out to DarkGray
                spans.push(Span::styled(ch, Style::default().fg(Color::DarkGray)));
            }
        }

        // Combine spans into a single Line, then into a Cell
        Cell::from(Line::from(spans))
    }
}

impl From<SignalStrength> for Cell<'static> { 
    fn from(signal: SignalStrength) -> Self { 
        signal.to_table_cell()
    }
    
}
