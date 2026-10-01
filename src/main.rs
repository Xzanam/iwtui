mod ui;

mod app;
use anyhow::Result;


#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut app = app::App::default();

    ratatui::run(|terminal| app.run(terminal));

    Ok(())
}
