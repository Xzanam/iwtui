pub struct NetworkBackend { 
    connection :  zbus::Connection 

}

impl NetworkBackend { 
    
    pub async fn new() -> anyhow::Result<Self>{ 

        let connection =  zbus::Connection::system().await?;

        Ok(NetworkBackend { 
            connection
        })
    }

    pub async fn  scan(){ 
        todo!()
    }
    
    pub async fn connect() { 
        todo!()
    }
    
    pub async  fn disconnect() { 
        todo!()
    }
}

 
