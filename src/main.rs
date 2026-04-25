mod api;
mod auth;
mod env;
mod error;
mod result;
mod rpc;
mod stats;
use crate::api::{Api, ApiState};
use crate::auth::PasswordStore;
use crate::env::CONFIG;
use crate::rpc::RpcStorage;
use crate::stats::Monitor;
use parking_lot::RwLock;
use result::Result;
use std::path::Path;
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging and load environment variables
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Load environment variables and panic if critical ones are missing or invalid
    dotenvy::dotenv().ok();
    let env = &*CONFIG;

    // Create a new password store for handling API authentication
    // For now we just have a single admin user, but this can be extended in the future
    // to use a more sophisticated user management system if needed.
    let pw_store = Arc::new(PasswordStore::new(Path::new("pw")));

    // Handle --setpass: prompt for password, hash and store, then exit
    if std::env::args().any(|a| a == "--setpass") {
        pw_store.setup_password()?;
        return Ok(());
    }

    // Initialize the rpc storage
    let rpc_storage = Arc::new(RwLock::new(RpcStorage::load(Path::new(&env.rpcs_path))?));
    
    // Create the stats monitor
    let monitor = Arc::new(Monitor::new(rpc_storage.clone()));

    // Start the monitor in the background to periodically update stats
    monitor.clone().start();

    // Create the API state and start the server
    let state = Arc::new(ApiState::new(rpc_storage, monitor, pw_store));
    let app = Api::new(state).router();

    // Bind the server to the specified port and start listening for requests
    let listener = TcpListener::bind(format!("0.0.0.0:{}", env.port)).await?;
    tracing::info!("Server listening on http://0.0.0.0:{}", env.port);
    
    // Serve the app using the bound listener
    axum::serve(listener, app).await?;
    Ok(())
}
