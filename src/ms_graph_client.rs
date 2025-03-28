mod access_token_response;
mod authentication;
mod oauth2_authorization_server;

use std::{collections::HashMap, error, sync::LazyLock};

use chrono::{Duration, Utc};
use log::{debug, trace};
use oauth2_authorization_server::OAuth2AuthorizationServer;
use reqwest::Client;
use url::Url;

use self::{access_token_response::AccessTokenResponse, authentication::Authentication};

/// Microsoft Graph APIs base URL.
static MS_GRAPH_BASE_URL: LazyLock<Url> = LazyLock::new(|| {
    Url::parse("https://graph.microsoft.com/v1.0/")
        .expect("failed to parse Microsoft Graph base URL")
});

/// Client used to interact with Microsoft Graph APIs.
pub struct MsGraphClient {
    /// Access token used to authenticate with Microsoft Graph APIs.
    access_token: String,

    /// HTTP client used to interact with Microsoft Graph APIs.
    http_client: Client,
}

impl MsGraphClient {
    /// Create a new instance of the MsGraphClient.
    pub async fn new(authentication: Authentication) -> Result<Self, Box<dyn error::Error>> {
        trace!("Initializing MS Graph Client...");

        let authorization_code = MsGraphClient::authorization_code(
            authentication.tenant_id.clone(),
            authentication.client_id.clone(),
        )
        .await?;

        let graph_client = Self {
            access_token: MsGraphClient::access_token(&authentication, authorization_code).await?,
            http_client: Client::new(),
        };

        debug!("MS Graph Client has been initialized!");

        Ok(graph_client)
    }

    /// Send a HTTP GET request to the given Microsoft Graph API path.
    pub async fn get(&self, path: &str) -> Result<String, Box<dyn error::Error>> {
        let path = path.trim_start_matches('/');
        let url = MS_GRAPH_BASE_URL.join(path)?;

        trace!("Calling GET {}", &url);

        let response = self
            .http_client
            .get(url.to_string())
            .bearer_auth(self.access_token.clone())
            .send()
            .await?;

        debug!("GET {} - {}", &url, response.status());

        Ok(response.text().await?)
    }

    /// Get an access token, on behalf of the user, that will be used to authenticate with Microsoft
    /// Graph APIs.
    async fn access_token(
        authentication: &Authentication,
        authorization_code: String,
    ) -> Result<String, Box<dyn error::Error>> {
        let request_url = format!(
            "https://login.microsoftonline.com/{}/oauth2/v2.0/token",
            authentication.tenant_id,
        );

        let mut request_params = HashMap::new();
        request_params.insert("client_id", authentication.client_id.clone());
        request_params.insert("scope", "offline_access User.Read".into());
        request_params.insert("code", authorization_code);
        request_params.insert(
            "redirect_uri",
            "http://localhost:8383/oauth2/redirect".into(),
        );
        request_params.insert("grant_type", "authorization_code".into());
        request_params.insert("client_secret", authentication.client_secret.clone());

        trace!("Attempting to retrieve access token...");

        let http_client: Client = Client::new();
        let response = http_client
            .post(request_url)
            .form(&request_params)
            .send()
            .await?
            .json::<AccessTokenResponse>()
            .await?;

        debug!(
            "Access token has been retrieved! It expires at {}.",
            Utc::now() + Duration::seconds(response.expires_in)
        );

        Ok(response.access_token)
    }

    /// Get an authorization code after user gives authorization.
    async fn authorization_code(
        tenant_id: String,
        client_id: String,
    ) -> Result<String, Box<dyn error::Error>> {
        let oauth2_authorization_server = OAuth2AuthorizationServer::start()?;
        let security_state = rand::random::<u32>();
        let request_url = Url::parse(&format!(
            "https://login.microsoftonline.com/{}/oauth2/v2.0/authorize?\
            client_id={}\
            &response_type=code\
            &redirect_uri=http://localhost:8383/oauth2/redirect\
            &response_mode=query\
            &scope=offline_access User.Read\
            &state={}",
            &tenant_id, &client_id, &security_state
        ))
        .expect("failed to parse OAuth2 authorization code URL");

        println!(
            "\nPlease navigate to the OAuth2 authorize URL below.\n{}\n\nWaiting for authorize redirect...\n",
            request_url
        );

        Ok(oauth2_authorization_server
            .authorization_code(security_state)
            .await?)
    }
}
