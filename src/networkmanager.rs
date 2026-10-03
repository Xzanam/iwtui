use tokio::sync::mpsc;
use backend::NetworkBackend;
use message::{NetworkCommand, NetworkEvent};

pub mod message;
pub mod models;
//temporary additions
pub mod backend;


pub struct  NetworkManager{
    backend : NetworkBackend, //Network Backend wrapper around zbus proxies and suitable functions
    command_rx: mpsc::Receiver<NetworkCommand>, //receives network commands from UI / Scheduled refreshes
    event_tx: mpsc::Sender<NetworkEvent>,
}
impl  NetworkManager{
    pub async fn new(
        command_rx: mpsc::Receiver<NetworkCommand>,
        event_tx: mpsc::Sender<NetworkEvent>,
    ) -> Self {
        
        let backend = NetworkBackend::new().await.unwrap();
        NetworkManager{
            backend, 
            command_rx,
            event_tx,
        }
    }

    pub async fn run(&mut self) {
        while let Some(command) = self.command_rx.recv().await {
            match command {
                NetworkCommand::Scan => {}
                NetworkCommand::Connect { ssid, password } => {}
                NetworkCommand::Disconnect => {}
                _ => {}
            }
        }
    }
}