pub mod wireless;
pub mod access_point;

pub mod statistics;

use zbus::{Result, proxy, zvariant::OwnedObjectPath};


#[proxy(
    interface = "org.freedesktop.NetworkManager",
    default_service = "org.freedesktop.NetworkManager",
    default_path = "/org/freedesktop/NetworkManager"
)]
pub trait NetworkManager {
    /// Returns object paths of all network devices known to the system.
    fn get_devices(&self) -> Result<Vec<OwnedObjectPath>>;
}