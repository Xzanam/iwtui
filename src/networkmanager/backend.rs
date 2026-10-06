use std::default;

use nmrs::ConnectByUuidConfig;


pub struct NetworkBackend { 
    nm : nmrs::NetworkManager, 
    snapshot: nmrs::NetworkSnapshot

}


pub struct WifiNetwork { 
    pub ssid : String 

}

impl From<nmrs::Network> for WifiNetwork { 
    fn from(item : nmrs::Network) ->   Self {

        WifiNetwork { 
            ssid : item.ssid
            //Need to merge this with  UI Model
        }
     }

}

impl NetworkBackend { 
    
    pub async fn new() -> anyhow::Result<Self>{ 

        let  nm = nmrs::NetworkManager::new().await?;
        let snapshot = nm.snapshot().await?;
        Ok(NetworkBackend {  
            nm, 
            snapshot

        })
    }

    pub async fn  scan(&self) -> anyhow::Result<Vec<WifiNetwork>> { 
        self.nm.scan_networks(None).await?;

        let networks =  self.nm.list_networks(None).await?;

        Ok(networks.into_iter().map(WifiNetwork::from).collect())
    }
    
    pub async fn  get_networks(&self) -> Option<Vec<WifiNetwork>> { 

        let networks = self.nm.list_networks(None).await.unwrap();

        if networks.is_empty()  { return  Option::None; }   

        Some(networks.into_iter().map(Into::<WifiNetwork>::into).collect::<Vec<_>>())
    }
    
    pub async fn connect(&self, network : &WifiNetwork)  { 
        if let Some(network) =  self.snapshot.saved_connections.iter().find(|conn| conn.id == network.ssid) { 
           match self.nm.connect_by_uuid(&network.uuid, ConnectByUuidConfig::default()).await { 
                Ok(_) => {
                    //Send  NetworkEvent for  ConnectionSuccessful
                    println!("Connected to network {}", network.id)
                     
                }, 
                Err(_) => {
                    //Send NetworkEvent for ConnectionFailed with reason/ errormsg
                    println!("Failed connecting to network {}", network.id)
                }
           }
        }
        else  { 
            //Send NetworkEvent for AuthenticationRequired
            println!("Network {} not found or you need password", network.ssid);
        }

        
    }
    
    pub async  fn disconnect() { 
        todo!()
    }
}

 
