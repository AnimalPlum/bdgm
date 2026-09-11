use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

use anyhow::{Context, Result};
use axum::Router;
use bimap::BiMap;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

pub(crate) async fn create_listener(port: Option<u16>) -> Result<TcpListener> {
    match port {
        Some(port) => Ok(TcpListener::bind(("127.0.0.1", port))
            .await
            .with_context(|| format!("failed to bind persisted port {}", port))?),
        None => Ok(TcpListener::bind("127.0.0.1:0")
            .await
            .context("failed to bind free port")?),
    }
}

pub(crate) async fn serve(listener: TcpListener, directory: PathBuf) -> Result<()> {
    let app = Router::new().fallback_service(ServeDir::new(directory));

    axum::serve(listener, app).await.context("Server crashed")?;

    Ok(())
}

pub(crate) fn get_portlist_path(data_dir: &PathBuf) -> PathBuf {
    data_dir.join("ports.json")
}

pub(crate) fn load_ports(data_dir: &PathBuf) -> Result<BiMap<String, u16>> {
    let path = get_portlist_path(data_dir);

    if !path.exists() {
        return Ok(BiMap::new());
    }

    let mut file = File::open(path)?;
    let mut json = String::new();

    file.lock_shared()?;

    match file.read_to_string(&mut json) {
        Ok(_) => serde_json::from_str(&json).map_err(|e| e.into()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(BiMap::new()),
        Err(e) => Err(e.into()),
    }
}

pub(crate) fn save_ports(ports: BiMap<String, u16>, data_dir: &PathBuf) -> Result<()> {
    let path = get_portlist_path(data_dir);
    let json = serde_json::to_string_pretty(&ports)?;
    let mut file = File::options().write(true).create(true).open(path)?;

    file.lock()?;
    write!(file, "{json}")?;

    Ok(())
}
