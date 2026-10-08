mod ui;

mod app;
mod networkmanager;

use anyhow::Result;

use crate::networkmanager::backend::NetworkBackend;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;



    let network = NetworkBackend::spawn().await.unwrap();

    let mut app = app::App::new(network.command_tx, network.event_rx);

    ratatui::run(|terminal| app.run(terminal));

    Ok(())
}
