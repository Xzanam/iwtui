
#[derive(Default, Debug)]
pub enum ConnectionStatus {
    Connected,
    #[default]
    NotConnected,
}

#[derive(Default, Debug)]
pub struct Networks {
    pub id : u16, 
    pub ssid: String,
    pub security: String,
}