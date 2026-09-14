use super::models::WifiNetwork;
pub enum NetworkCommand {
    Scan,
    Connect { ssid: String, password: String },
    Disconnect,
}

pub enum NetworkEvent {
    NetworksUpdated(Vec<WifiNetwork>),
}
