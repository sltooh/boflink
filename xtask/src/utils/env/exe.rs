use std::{ffi::OsString, sync::LazyLock};

pub fn cargo() -> OsString {
    static CARGO_EXE: LazyLock<OsString> =
        LazyLock::new(|| std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()));

    LazyLock::force(&CARGO_EXE).to_os_string()
}

pub fn rustc() -> OsString {
    static RUSTC_EXE: LazyLock<OsString> =
        LazyLock::new(|| std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into()));

    LazyLock::force(&RUSTC_EXE).to_os_string()
}
