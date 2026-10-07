use super::{message::{NetworkCommand, NetworkEvent}, models::{WifiNetwork}};


pub struct NetworkBackend {
    nm: nmrs::NetworkManager,
    snapshot: nmrs::NetworkSnapshot,
    command_rx: tokio::sync::mpsc::Receiver<NetworkCommand>,
    event_tx: tokio::sync::mpsc::Sender<NetworkEvent>,
}

impl From<nmrs::Network> for super::models::WifiNetwork {
    fn from(item: nmrs::Network) -> Self {
        WifiNetwork {
            ssid: item.ssid, //Need to merge this with  UI Model
        }
    }
}

impl NetworkBackend {
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
                    self.event_tx.send(NetworkEvent::ScanStarted);
                    let result = self.scan().await?;
                    self.event_tx.send(NetworkEvent::ScanCompleted(result));
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
