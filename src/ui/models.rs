
use std::fmt;

use crate::ui::models::wifisignal::SignalStrength;

pub mod wifisignal;

#[derive(Default, Debug)]
pub enum ConnectionStatus {
    Connected,
    #[default]
    NotConnected,
}

#[derive(Default, Debug)]
pub struct WifiNetwork {
    pub ssid: String,
    pub security: String,
    pub band: WifiBand,
    pub strength : SignalStrength
}

#[derive(Default, Debug)]
pub enum WifiBand {
    #[default]
    B24,
    B5,
    B6,
    Unknown(u32)
}

impl fmt::Display for WifiBand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::B24 => write!(f, "2.4 GHz"),
            Self::B5 => write!(f, "5 GHz"),
            Self::B6 => write!(f, "6 GHz"),
            Self::Unknown(mhz) => write!(f, "{} MHz", mhz),
        }
    }
    
}


impl WifiBand { 
    
    pub fn as_str(&self) -> &'static str { 
        match  self { 
            Self::B24 => "2.4 Ghz", 
            Self::B5 => "5 Ghz", 
            Self::B6 => "6Ghz", 
            Self::Unknown(u32) => "Unknown"
             
        }
    }
    
}
