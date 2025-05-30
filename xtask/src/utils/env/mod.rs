use std::{
    error::Error,
    path::{Path, PathBuf},
    sync::LazyLock,
};

use exe::rustc;

use super::shell::check_output_projdir;

pub mod exe;

pub fn project_root() -> std::io::Result<&'static Path> {
    static MANIFEST_DIR: LazyLock<Option<&'static Path>> = LazyLock::new(|| {
        let path = Path::new(&env!("CARGO_MANIFEST_DIR")).ancestors().nth(1);

        if let Some(path) = path {
            if !path.join("Cargo.toml").is_file() {
                return None;
            }

            if !path.join(".git").is_dir() {
                return None;
            }
        }

        path
    });

    LazyLock::force(&MANIFEST_DIR)
        .ok_or_else(|| std::io::Error::other("could not detect project root"))
}

pub fn cargo_home() -> std::io::Result<PathBuf> {
    static CARGO_HOME: LazyLock<Option<PathBuf>> = LazyLock::new(|| {
        std::env::var_os("CARGO_HOME")
            .map(PathBuf::from)
            .or_else(|| {
                if cfg!(unix) {
                    std::env::var_os("HOME")
                } else if cfg!(windows) {
                    std::env::var_os("USERPROFILE")
                } else {
                    None
                }
                .map(|home| Path::new(&home).join(".cargo"))
            })
    });

    LazyLock::force(&CARGO_HOME)
        .clone()
        .ok_or_else(|| std::io::Error::other("could not find CARGO_HOME"))
}

pub fn install_prefix() -> PathBuf {
    static INSTALL_PREFIX: LazyLock<PathBuf> = LazyLock::new(|| {
        std::env::var_os("PREFIX")
            .map(PathBuf::from)
            .or_else(|| std::env::var_os("HOME").map(|home| Path::new(&home).join(".local")))
            .unwrap_or_else(|| PathBuf::from("/usr/local"))
    });

    LazyLock::force(&INSTALL_PREFIX).clone()
}

pub fn install_libexec() -> PathBuf {
    install_prefix().join("libexec")
}

pub fn rustc_host() -> Result<String, Box<dyn Error>> {
    let output = check_output_projdir(rustc(), ["--version", "--verbose"])?;

    output
        .lines()
        .find_map(|line| line.strip_prefix("host: "))
        .map(String::from)
        .ok_or_else(|| "could not find rustc host".into())
}
