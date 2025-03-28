mod ms_graph_client;

use std::{
    error,
    fs::File,
    io::{BufReader, Write},
};

use chrono::Utc;
use log::{LevelFilter, debug, info, trace};

use crate::ms_graph_client::MsGraphClient;

/// Application entry point.
#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    initialize_logger();

    trace!("Reading authentication details from file...");

    let authentication =
        serde_json::from_reader(BufReader::new(File::open("./.data/authentication.json")?))?;

    debug!("Authentication details have been read!");

    let graph_client = MsGraphClient::new(authentication).await?;

    let user = graph_client.get("/me").await?;

    info!("Welcome {}!", user);

    Ok(())
}

/// Initialize environment logger used throughout the application.
fn initialize_logger() {
    env_logger::Builder::new()
        .format(|buffer, record| {
            writeln!(
                buffer,
                "{} [{}] - {}",
                Utc::now().format("%Y-%m-%d %H:%M:%SZ"),
                record.level(),
                record.args(),
            )
        })
        .filter_module("ms_graph_auto_mail", LevelFilter::Trace)
        .init();

    debug!("Environment logger has been initialized!");
}
