#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use std::{env, sync::Arc};
use crate::store::Store;
mod simulator;
mod store;
mod error;
mod prelude;

use crate::prelude::*;

#[macro_use]
extern crate log;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
#[tokio::main]
async fn main() -> Result<()> {
    env::set_var("RUST_LOG", "error");
    env_logger::init();
    info!("Program Start Up");
    let store = Store::new().await?;
    store.setup().await;

    let store = Arc::new(store);
    tauri::Builder::default()
        .manage(store)
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
