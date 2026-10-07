use super::models::WifiNetwork;

#[non_exhaustive]
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


