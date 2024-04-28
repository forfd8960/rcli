use std::path::{Path, PathBuf};

pub mod base64;
pub mod csv;
pub mod genpass;
pub mod opts;
pub mod text;

fn verify_input(f: &str) -> std::result::Result<String, &'static str> {
    if f == "-" || std::path::Path::new(f).exists() {
        Ok(f.into())
    } else {
        Err("File is not exists")
    }
}

fn verify_output(f: &str) -> std::result::Result<PathBuf, &'static str> {
    let p = Path::new(f);
    if p.exists() && p.is_dir() {
        Ok(p.into())
    } else {
        Err("Path is not exists or is not a dir")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input() {
        assert_eq!(verify_input("-"), Ok("-".into()));
        assert_eq!(verify_input("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_input("unknow_file"), Err("File is not exists"));
    }
}
