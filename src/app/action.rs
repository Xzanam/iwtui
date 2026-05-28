#[derive(Debug, Clone)]
pub enum Action {
    MoveUp,
    MoveDown,
    ConnectSelected,
    RescanNetworks,
    OpenPasswordPrompt,
    ClosePopup,
    Quit,
}