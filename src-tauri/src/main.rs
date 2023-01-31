#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use std::{env, sync::Arc};
use tokio::sync::RwLock;
use crate::store::Store;
mod simulator;
mod store;
mod error;
mod prelude;
mod ipc;

use crate::ipc::{retrieve_all_items, retrieve_all_units, retrieve_item_from_id, retrieve_unit_from_id, retrieve_all_item_ids, retrieve_all_unit_ids, update_unit, update_item, submit_board, fetch_board, simulate_x_ticks, update_outcome, fetch_outcomes};
use crate::prelude::*;

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() -> Result<()> {
    env::set_var("RUST_LOG", "error");
    env_logger::init();
    info!("Program Start Up");
    let store = Store::new().await?;
    if store.setup().await.is_ok() {
        let store = Arc::new(RwLock::new(store));
        tauri::Builder::default()
            .manage(store)
            .invoke_handler(tauri::generate_handler![
                retrieve_all_items,
                retrieve_all_units,
                retrieve_item_from_id,
                retrieve_unit_from_id,
                retrieve_all_item_ids,
                retrieve_all_unit_ids,
                update_unit,
                update_item,
                submit_board,
                fetch_board,
                simulate_x_ticks,
                update_outcome,
                fetch_outcomes
                
            ])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
        Ok(())
    }
    else {
        Err(Error::DatabaseError("Failure to Start Up"))
    }
    
}
