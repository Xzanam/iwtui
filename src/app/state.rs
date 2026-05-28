
#[derive(Clone)]
struct WifiNetwork {
    ssid: String,
    security: String,
    signal: String,
}

#[derive(Clone, PartialEq)]
pub enum View {
    NetworkList,
    PasswordPrompt,
    Connecting,
    ErrorPopup,
}


pub struct AppState {
    pub networks: Vec<WifiNetwork>,

    pub selected: usize,

    pub status: String,

    pub current_view: View,

    pub should_quit: bool,
}