use std::{
    error::{self},
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use log::{debug, error, trace};

/// Server used to receive OAuth2 Authorization redirect requests.
pub struct OAuth2AuthorizationServer {
    /// TCP socket server used to listen for connections.
    tcp_listener: TcpListener,
}

impl OAuth2AuthorizationServer {
    /// Start the OAuth2 Authorization Server.
    pub fn start() -> Result<Self, Box<dyn error::Error>> {
        const HOST_ADDRESS: &str = "127.0.0.1:8383";

        trace!(
            "Starting OAuth2 Authorization Server at {}...",
            HOST_ADDRESS
        );

        let tcp_listener = TcpListener::bind(HOST_ADDRESS)?;

        debug!(
            "OAuth2 Authorization Server is listening at {}!",
            HOST_ADDRESS
        );

        Ok(Self { tcp_listener })
    }

    /// Get an authorization code after receiving the authorize redirect request.
    pub async fn authorization_code(
        &self,
        security_state: u32,
    ) -> Result<String, Box<dyn error::Error>> {
        for tcp_stream in self.tcp_listener.incoming() {
            match tcp_stream {
                Ok(tcp_stream) => {
                    if let Some(authorization_code) =
                        Self::handle_request(tcp_stream, security_state)
                    {
                        return Ok(authorization_code);
                    };
                }
                Err(error) => error!("Failed to accept connection: {}", error),
            }
        }

        error!("Failed to get authorization code before TCP listener was stopped.");

        Ok(String::default())
    }

    /// Handle HTTP requests by parsing them and taking the necessary action based on the contents.
    fn handle_request(mut stream: TcpStream, security_state: u32) -> Option<String> {
        let mut buffer = [0; 3072];
        stream
            .read(&mut buffer)
            .expect("failed to read stream into buffer");

        let request = String::from_utf8_lossy(&buffer);
        let request = request.lines().next().unwrap_or("");

        const REDIRECT_PATH: &str = "GET /oauth2/redirect?";

        if request.starts_with(REDIRECT_PATH) {
            let last_space_index = request
                .rfind(" ")
                .expect("failed to find last space in request");
            let request_parameters = &request[REDIRECT_PATH.len()..last_space_index];
            let mut authorization_code = String::default();
            let mut is_secure = false;

            for pair in request_parameters.split('&') {
                let mut pair = pair.split('=');

                if let (Some(key), Some(value)) = (pair.next(), pair.next()) {
                    if key == "code" {
                        authorization_code = value.to_string();
                    } else if key == "state" {
                        let value = value
                            .parse::<u32>()
                            .expect("failed to parse request state as an u32");

                        is_secure = value == security_state;
                    }

                    if !authorization_code.is_empty() && is_secure {
                        let response = "HTTP/1.1 200 OK\r\nContent-Length: 53\r\n\r\nAuthorization Granted! You may now close this window.";
                        stream
                            .write(response.as_bytes())
                            .expect("failed to write response");

                        return Some(authorization_code);
                    }
                }
            }
        }

        None
    }
}
