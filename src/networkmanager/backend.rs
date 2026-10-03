pub struct NetworkBackend { 

    nm : nmrs::NetworkManager

}


struct WifiNetworks { 

}
impl NetworkBackend { 

    
    pub async fn new() -> anyhow::Result<Self>{ 

        let  nm = nmrs::NetworkManager::new().await?;
        Ok(NetworkBackend {  
            nm
        })
    }

    pub async fn  scan(&self) -> anyhow::Result<bool> { 
        let networks = self.nm.scan_networks(None).await?;
        
        Ok(true)
    }
    
    pub async fn  get_networks(&self) -> Option<Vec<WifiNetworks>> { 
        
        Some(vec![])
        
    }
    
    pub async fn connect() { 
        todo!()
    }
    
    pub async  fn disconnect() { 
        todo!()
    }
}

 
