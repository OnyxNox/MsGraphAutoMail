use serde::Deserialize;

/// Represents a collection of details used to authenticate with Microsoft Graph APIs.
#[derive(Deserialize)]
pub struct Authentication {
    pub client_id: String,

    pub client_secret: String,

    pub tenant_id: String,
}
