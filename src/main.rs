mod networkmanager;

use std::thread;

use anyhow::Result;
use iwtui::networkmanager::{
     message::{NetworkCommand, NetworkEvent}, models::WifiNetwork
};
use tokio::sync::mpsc;


#[tokio::main]
async fn main() -> Result<()> {
    /*  let (command_tx, command_rx) = tokio::sync::mpsc::channel(32);
    let (event_tx, event_rx) = tokio::sync::mpsc::channel(32);
    // let (input_tx, input_rx)  = tokio::sync::mpsc::channel(32);

    let app = App::new(command_tx, event_rx);

    let mut  network_manager= NetworkManager::new(command_rx, event_tx);
    thread::spawn(async move || {network_manager.run().await});

     */




    Ok(())
}
