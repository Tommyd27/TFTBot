use tauri::command;

use crate::simulator::{champions::Champion, item::Item};



#[command]
pub async fn retrieve_unit() -> Option<Champion> {
    
}