
use super::{message::{NetworkCommand, NetworkEvent}};
use crate::ui::models::{WifiBand, WifiNetwork};
use tokio::sync::mpsc;


#[derive(Debug)]
pub struct NetworkBackend {
    nm: nmrs::NetworkManager,
    snapshot: nmrs::NetworkSnapshot,
    command_rx: tokio::sync::mpsc::Receiver<NetworkCommand>,
    event_tx: tokio::sync::mpsc::Sender<NetworkEvent>,
}

impl From<nmrs::Network> for WifiNetwork {
    fn from(item: nmrs::Network) -> Self {
        WifiNetwork {
            ssid: item.ssid, //Need to merge this with  UI Model
            security : "WPA".to_string(), 
            band : WifiBand::B5, 
            strength : item.strength.unwrap().into()
        }
    }
}

pub struct NetworkHandle { 
    pub command_tx : mpsc::Sender<NetworkCommand>, 
    pub event_rx :  mpsc::Receiver<NetworkEvent>

}
impl NetworkBackend {
    
    pub async  fn spawn() -> anyhow::Result<NetworkHandle> { 
    let (command_tx, command_rx) = tokio::sync::mpsc::channel(1024);
    let (event_tx, event_rx) = tokio::sync::mpsc::channel(1024);
        
        let mut backend = Self::new(command_rx, event_tx).await?;
        
        tokio::spawn(async  move { 
            if let Err(err) = backend.run().await { 
                eprintln!("Network Backend Stopped : {}" , err);
            }
            
        });
        
        Ok(NetworkHandle { command_tx, event_rx })
    }
    pub async fn new(
        command_rx: tokio::sync::mpsc::Receiver<NetworkCommand>,
        event_tx: tokio::sync::mpsc::Sender<NetworkEvent>,
    ) -> anyhow::Result<Self> {
        let nm = nmrs::NetworkManager::new().await?;
        let snapshot = nm.snapshot().await?;
        Ok(NetworkBackend {
            nm,
            snapshot,
            command_rx,
            event_tx
        })
    }


    pub async fn run(&mut self) -> anyhow::Result<()> {
        while let Some(command) = self.command_rx.recv().await {
            match command {
                NetworkCommand::Scan => {
                    self.event_tx.send(NetworkEvent::ScanStarted).await?;
                    let result = self.scan().await?;
                    self.event_tx.send(NetworkEvent::ScanCompleted(result)).await?;
                }, 

                _ => {}
            }
        }

        Ok(())
    }
    

    pub async fn scan(&self) -> anyhow::Result<Vec<WifiNetwork>> {
        self.nm.scan_networks(None).await?;
        let networks = self.nm.list_networks(None).await?;

        Ok(networks.into_iter().map(WifiNetwork::from).collect())

    }

    pub async fn get_networks(&self) -> Option<Vec<WifiNetwork>> {
        let networks = self.nm.list_networks(None).await.unwrap();

        if networks.is_empty() {
            return Option::None;
        }

        Some(
            networks
                .into_iter()
                .map(Into::<WifiNetwork>::into)
                .collect::<Vec<_>>(),
        )
    }

    pub async fn connect(&self, network: &WifiNetwork) {
        if let Some(network) = self
            .snapshot
            .saved_connections
            .iter()
            .find(|conn| conn.id == network.ssid)
        {
            match self
                .nm
                .connect_by_uuid(&network.uuid, nmrs::ConnectByUuidConfig::default())
                .await
            {
                Ok(_) => {
                    //Send  NetworkEvent for  ConnectionSuccessful
                    println!("Connected to network {}", network.id)
                }
                Err(_) => {
                    //Send NetworkEvent for ConnectionFailed with reason/ errormsg
                    println!("Failed connecting to network {}", network.id)
                }
            }
        } else {
            //Send NetworkEvent for AuthenticationRequired
            println!("Network {} not found or you need password", network.ssid);
        }
    }

    pub async fn disconnect() {
        todo!()
    }
}
