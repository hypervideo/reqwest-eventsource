use std::env::var;

fn main() {
    if var("CARGO_CFG_TARGET_ARCH").unwrap_or_default() == "wasm32"
        && var("CARGO_CFG_TARGET_VENDOR").unwrap_or_default() == "unknown"
        && var("CARGO_CFG_TARGET_ENV").unwrap_or_default() == ""
        && var("CARGO_FEATURE_WASM_BINDGEN").is_err()
    {
        println!("cargo:warning=Compiling for wasm32-unknown-unknown but without wasm-bindgen feature enabled. Trying to access futures-timer functionality will cause panics.");
    }
}
