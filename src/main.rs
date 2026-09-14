mod networkmanager;

use std::thread;

use anyhow::Result;
use iwtui::networkmanager::{
    access_point::{AccessPointExt, AccessPointProxy}, manager::NetworkManagerProxy, message::{NetworkCommand, NetworkEvent}, models::WifiNetwork, wireless::{DeviceProxy, DeviceWirelessProxy},
};
use tokio::sync::mpsc;

pub struct App {
    pub command_tx: tokio::sync::mpsc::Sender<NetworkCommand>,
    pub event_rx: tokio::sync::mpsc::Receiver<NetworkEvent>,
    pub networks: Vec<WifiNetwork>,
    pub selected: usize,
    pub is_running: bool,
}

impl App {
    pub fn new(
        command_tx: mpsc::Sender<NetworkCommand>,
        event_rx: mpsc::Receiver<NetworkEvent>,
    ) -> Self {
        App {
            command_tx,
            event_rx,
            networks: Vec::new(),
            selected: 0,
            is_running: true,
        }
    }

    pub async fn send_test_command_scan(&self) -> Result<()> {
        let networks = self.command_tx.send(NetworkCommand::Scan).await?;
        Ok(())
    }
}

pub struct NetworkManager {
    command_rx: mpsc::Receiver<NetworkCommand>,
    event_tx: mpsc::Sender<NetworkEvent>,
}
impl NetworkManager {
    pub fn new(
        command_rx: mpsc::Receiver<NetworkCommand>,
        event_tx: mpsc::Sender<NetworkEvent>,
    ) -> Self {
        NetworkManager {
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
#[tokio::main]
async fn main() -> Result<()> {
    /*  let (command_tx, command_rx) = tokio::sync::mpsc::channel(32);
    let (event_tx, event_rx) = tokio::sync::mpsc::channel(32);
    // let (input_tx, input_rx)  = tokio::sync::mpsc::channel(32);

    let app = App::new(command_tx, event_rx);

    let mut  network_manager= NetworkManager::new(command_rx, event_tx);
    thread::spawn(async move || {network_manager.run().await});

     */
    let connection = zbus::Connection::system().await?;

    let nm = NetworkManagerProxy::builder(&connection).build().await?;
    
    let devices = nm.get_devices().await?;
    
    for device_path in devices { 

        let device = DeviceProxy::builder(&connection).path(device_path.clone())?.build().await?;
        let device_type = device.device_type().await?;
        
        if device_type == 2  { 
            let wifi = DeviceWirelessProxy::builder(&connection).path(device_path)?.build().await?;
            
            let access_point_paths = wifi.get_all_access_points().await?;
            
            for path in access_point_paths { 
                
                let access_point = AccessPointProxy::builder(&connection).path(path)?.build().await?;
                
                println!("{}" , access_point.get_ssid().await?);
            }
            
            break;
            
        }
    }



    Ok(())
}
