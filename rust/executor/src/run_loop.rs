use notify::{Watcher, RecursiveMode, recommended_watcher};
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::path::Path;

pub fn spawn_hot_reload(cfg_path: &str, reload_flag: Arc<AtomicBool>) {
    let path = cfg_path.to_string();
    let (tx, rx) = channel();
    
    // Create a watcher for file events
    let mut watcher = match recommended_watcher(move |res| {
        if let Ok(_) = res {
            let _ = tx.send(());
        }
    }) {
        Ok(w) => w,
        Err(e) => {
            log::error!("Failed to create watcher: {}", e);
            return;
        }
    };
    
    // Watch the config file for changes
    if let Err(e) = watcher.watch(Path::new(&path), RecursiveMode::Recursive) {
        log::error!("Failed to watch path {}: {}", path, e);
        return;
    }

    let reload_flag_clone = reload_flag.clone();
    std::thread::spawn(move || {
        for _ in rx {
            log::info!("🔄 Config changed – reloading");
            reload_flag_clone.store(true, Ordering::SeqCst);
            unsafe { libc::raise(libc::SIGHUP) }; // triggers graceful handler
        }
    });
    
    log::info!("Config hot-reload watcher started for {}", cfg_path);
}

pub fn handle_config_reload(config_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Reloading config from {}", config_path);
    
    // Here you would read and parse your config file
    // let config = std::fs::read_to_string(config_path)?;
    // let parsed_config = toml::from_str(&config)?;
    
    // Then apply the new configuration...
    
    log::info!("Config reload completed successfully");
    Ok(())
}
