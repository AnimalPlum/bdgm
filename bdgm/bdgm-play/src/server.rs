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

pub(crate) fn get_portlist_file(data_dir: &PathBuf) -> Result<File> {
    let path = data_dir.join("ports.json");
    let file = File::options().write(true).create(true).open(path)?;
    file.lock()?;
    Ok(file)
}

pub(crate) fn load_ports(file: &mut File) -> Result<BiMap<String, u16>> {
    let mut json = String::new();
    match file.read_to_string(&mut json) {
        Ok(_) => serde_json::from_str(&json).map_err(|e| e.into()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(BiMap::new()),
        Err(e) => Err(e.into()),
    }
}

pub(crate) fn save_ports(ports: BiMap<String, u16>, file: &mut File) -> Result<()> {
    let json = serde_json::to_string_pretty(&ports)?;
    write!(file, "{json}")?;
    Ok(())
}
