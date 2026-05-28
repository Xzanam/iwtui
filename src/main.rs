use std::io;

use color_eyre::Result;
use crossterm::{execute, terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode}};
use ratatui::{Terminal, backend::CrosstermBackend};

fn main() -> Result<()> 
{
    color_eyre::install()?;
    enable_raw_mode()?;
    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen);

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res =  execute!(terminal.backend_mut(), LeaveAlternateScreen);
    disable_raw_mode()?;

    Ok(())
}
