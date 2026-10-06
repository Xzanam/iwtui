use anyhow::Result;
use nmrs::NetworkManager;

use crate::networkmanager::backend::{NetworkBackend, WifiNetwork};

mod networkmanager;



#[non_exhaustive]
pub enum NetworkEvent {
    ScanStarted,
    ScanCompleted(Vec<WifiNetwork>),
    ScanFailed(anyhow::Error),

    Connecting {
        ssid: String,
    },

    AuthenticationRequired {
        ssid: String,
    },

    Connected {
        ssid: String,
    },

    ConnectionFailed {
        ssid: String,
        error: String,
    },

    Disconnected
}

pub enum NetworkCommand {
    Scan,

    Connect {
        network: WifiNetwork,
    },

    ConnectWithPassword {
        network: WifiNetwork,
        password: String,
    },

    Disconnect,
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let networkworker = NetworkBackend::new().await?;
    let wifi_network =  WifiNetwork { ssid : "SegmentationFault_2.4".to_string()};

     
    networkworker.connect(&wifi_network).await;
    
    let ( event_tx, mut event_rx) = tokio::sync::mpsc::channel(1024);
    tokio::spawn( async move { 
        while let Some(message) =   event_rx.recv().await { 

            match message { 
                NetworkCommand::Scan => println!("Scanning..."),
                NetworkCommand::Disconnect =>  println!("Disconneciting"), 
                _ => {
                }

            }
        }
    }) ;
    event_tx.send(NetworkCommand::Scan).await? ;
    Ok(())
}



