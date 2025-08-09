mod run_loop;

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use axum::{
    routing::get,
    Router,
    Json,
    http::StatusCode,
};
use serde_json::json;
use tokio::net::TcpListener;
use tokio::signal;
use signal_hook_tokio::Signals;
use futures_util::stream::StreamExt;

// Config structure that would be loaded from TOML
#[derive(Default, Debug, serde::Deserialize)]
struct Config {
    bot: BotConfig,
}

#[derive(Default, Debug, serde::Deserialize)]
struct BotConfig {
    name: String,
    rpc_url: Option<String>,
    #[serde(default = "default_gas_price")]
    gas_price: u64,
}

fn default_gas_price() -> u64 {
    200_000_000 // 0.2 Gwei
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    
    // Get config path from environment or use default
    let config_path = std::env::var("CONFIG_PATH").unwrap_or_else(|_| "config/bot.toml".to_string());
    
    // Load initial configuration
    let config = match load_config(&config_path) {
        Ok(config) => Arc::new(std::sync::RwLock::new(config)),
        Err(e) => {
            log::error!("Failed to load config: {}", e);
            return Err(e.into());
        }
    };
    
    // Set up reload flag for hot reloading
    let reload_flag = Arc::new(AtomicBool::new(false));
    
    // Set up hot reload file watcher
    run_loop::spawn_hot_reload(&config_path, reload_flag.clone());
    
    // Register signal handler for SIGHUP
    let reload_flag_clone = reload_flag.clone();
    let config_path_clone = config_path.clone();
    let config_clone = config.clone();
    
    tokio::spawn(async move {
        let mut signals = Signals::new(&[signal_hook::consts::signal::SIGHUP]).unwrap();
        
        while let Some(signal) = signals.next().await {
            if signal == signal_hook::consts::signal::SIGHUP {
                log::info!("Received SIGHUP signal");
                reload_flag_clone.store(true, Ordering::SeqCst);
                
                // Reload config
                match load_config(&config_path_clone) {
                    Ok(new_config) => {
                        let mut config_write = config_clone.write().unwrap();
                        *config_write = new_config;
                        log::info!("Configuration reloaded successfully");
                    },
                    Err(e) => {
                        log::error!("Failed to reload config: {}", e);
                    }
                }
                
                reload_flag_clone.store(false, Ordering::SeqCst);
            }
        }
    });
    
    // Setup HTTP server for health checks and metrics
    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/metrics", get(metrics_handler));
    
    // Bind to a socket
    let listener = match TcpListener::bind("0.0.0.0:9000").await {
        Ok(l) => l,
        Err(e) => {
            log::error!("Failed to bind to socket: {}", e);
            return Err(e.into());
        }
    };
    
    log::info!("HTTP server listening on 0.0.0.0:9000");
    
    // Run the HTTP server
    tokio::select! {
        result = axum::Server::from_tcp(listener.into_std()?)?.serve(app.into_make_service()) => {
            if let Err(e) = result {
                log::error!("HTTP server error: {}", e);
            } else {
                log::info!("HTTP server terminated");
            }
        },
        _ = signal::ctrl_c() => {
            log::info!("Received SIGINT, shutting down gracefully");
        }
    }
    
    log::info!("Application shutdown complete");
    Ok(())
}

// Function to load config from a TOML file
fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let content = match std::fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            log::error!("Could not read config file: {}", e);
            return Err(Box::new(e));
        }
    };
    
    match toml::from_str(&content) {
        Ok(config) => {
            log::info!("Loaded configuration from {}", path);
            Ok(config)
        },
        Err(e) => {
            log::error!("Failed to parse config: {}", e);
            Err(Box::new(e))
        }
    }
}

async fn health_handler() -> (StatusCode, Json<serde_json::Value>) {
    (StatusCode::OK, Json(json!({
        "status": "ok",
        "uptime": std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    })))
}

async fn metrics_handler() -> (StatusCode, Json<serde_json::Value>) {
    (StatusCode::OK, Json(json!({
        "health": "ok",
        "monitoring": "active"
    })))
}
