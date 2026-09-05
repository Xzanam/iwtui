use iwtui::networkmanager::{
    NetworkManagerProxy,
    access_point::AccessPointExt,
    wireless::{DeviceProxy, WirelessExt, WirelessProxy},
};

use anyhow::Result;

use zbus::Connection;

#[tokio::main]
async fn main() -> Result<()> {
    let connection = zbus::Connection::system().await?;

    let wireless = WirelessProxy::new(&connection).await?;

    let nm = NetworkManagerProxy::new(&connection).await?;
    let device_path = nm.get_devices().await?;

    for path in device_path {
        let device = DeviceProxy::builder(&connection)
                                                .path(path)?
                                                .build()
                                                .await?;
        
        let interface = device.interface().await?;
        println!("{}", interface);
    }

    // let ap_proxies = wireless.get_all_access_point_proxies().await?;

    // for (i, ap) in ap_proxies.into_iter().enumerate() {
    //     match ap.get_ssid().await {
    //         Ok(ssid) => {
    //             // 2. Print out the index and the string cleanly
    //             println!("AP {}: {}", i + 1, ssid); // i + 1 makes the user-facing list start at 1 instead of 0
    //         }
    //         Err(e) => {
    //             eprintln!("AP {}: ⚠️ Failed to fetch details: {:?}", i + 1, e);
    //         }
    //     }
    // }

    Ok(())
}
