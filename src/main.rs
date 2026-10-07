use anyhow::Result;
use nmrs::NetworkManager;

use crate::networkmanager::{backend::NetworkBackend, message::NetworkCommand, models::WifiNetwork};

mod networkmanager;




#[tokio::main]
async fn main() -> anyhow::Result<()> {

    
    let (command_tx, command_rx) = tokio::sync::mpsc::channel(1024);
    let (event_tx, event_rx) = tokio::sync::mpsc::channel(1024);
    
    let networkworker = NetworkBackend::new(command_rx, event_tx.clone()).await?;
    let wifi_network =  WifiNetwork { ssid : "SegmentationFault_2.4".to_string()};

     
    networkworker.connect(&wifi_network).await;
    
    Ok(())
}



