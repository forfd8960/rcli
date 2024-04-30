use std::path::Path;

pub fn process_http_serve(path: &Path, port: u16) -> anyhow::Result<()> {
    tracing::info!("serving: {:?} on port: {}", path, port);
    anyhow::Ok(())
}
