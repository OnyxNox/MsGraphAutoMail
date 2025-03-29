use serde::Deserialize;

/// Represents a HTTP response used to return a Microsoft Entra ID Application's access token.
#[derive(Deserialize)]
pub struct AccessTokenResponse {
    /// Access token used to authenticate with Microsoft Graph APIs.
    pub access_token: String,

    /// Number of seconds the access token is valid for.
    pub expires_in: i64,
}

/// Represents a collection of details used to authenticate with Microsoft Graph APIs.
#[derive(Deserialize)]
pub struct Authentication {
    /// Microsoft Entra ID Application (client) identifier used to authenticate with Microsoft Graph
    /// APIs.
    pub client_id: String,

    /// Microsoft Entra ID Application (client) secret used to authenticate with Microsoft Graph
    /// APIs.
    pub client_secret: String,

    /// Microsoft Entra ID tenant identifier used to authenticate with Microsoft Graph APIs.
    pub tenant_id: String,
}

/// Represents a Microsoft Entra user account.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    /// Given name (first name) of the user.
    pub given_name: String,
}
