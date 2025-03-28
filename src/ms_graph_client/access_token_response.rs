use serde::Deserialize;

/// Represents a HTTP response used to return a Microsoft Entra ID Application's access token.
#[derive(Deserialize)]
pub struct AccessTokenResponse {
    /// Access token used to authenticate with Microsoft Graph APIs.
    pub access_token: String,

    /// Number of seconds the access token is valid for.
    pub expires_in: i64,
}
