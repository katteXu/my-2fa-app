use crate::totp::TotpConfig;

pub struct AppState {
    pub config: Option<TotpConfig>,
}

impl AppState {
    pub fn new() -> Self {
        Self { config: None }
    }
}
