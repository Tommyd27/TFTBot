use crate::prelude::*;
use crate::simulator::champions::PlacedChampion;
use crate::simulator::{champions::Champion, item::Item};
use crate::store::Store;
use std::sync::Arc;
use tauri::{command, AppHandle, Manager, Wry};

fn get_store_from_state(connection: AppHandle<Wry>) -> Result<Arc<Store>> {
    Ok((*connection.state::<Arc<Store>>()).clone())
}

#[command]
pub async fn retrieve_unit_from_id(id: u8, connection: AppHandle<Wry>) -> Result<Option<Champion>> {
    if let Ok(store) = get_store_from_state(connection) {
        return store.fetch_champion_from_id(id).await
    }
    Err(Error::StoreError)
}

#[command]
pub async fn retrieve_item_from_id(id: u8, connection: AppHandle<Wry>) -> Result<Option<Item>> {
    //*Option<Champion> {
    if let Ok(store) = get_store_from_state(connection) {
        return store.fetch_item_from_id(id).await
    }
    Err(Error::StoreError)
}

#[command]
pub async fn retrieve_all_units(connection : AppHandle<Wry>) -> Result<Vec<Champion>> {
    if let Ok(store) = get_store_from_state(connection) {
        return store.fetch_champions().await
    }
    Err(Error::StoreError)
}

#[command]
pub async fn retrieve_all_items(connection : AppHandle<Wry>) -> Result<Vec<Item>> {
    if let Ok(store) = get_store_from_state(connection) {
        return store.fetch_items().await
    }
    Err(Error::StoreError)
}
#[command]
pub async fn retrieve_all_unit_ids(connection : AppHandle<Wry>) -> Result<Vec<u8>> {
    if let Ok(store) = get_store_from_state(connection) {
        return store.fetch_champions_ids().await
    }
    Err(Error::StoreError)
}
#[command]
pub async fn retrieve_all_item_ids(connection : AppHandle<Wry>) -> Result<Vec<u8>> {
    if let Ok(store) = get_store_from_state(connection) {
        return store.fetch_items_ids().await
    }
    Err(Error::StoreError)
}

#[command]
pub async fn update_unit(selected_unit : Champion, connection : AppHandle<Wry>) -> Result<()> {
    if let Ok(store) = get_store_from_state(connection) {
        return store.update_champion(selected_unit).await;
    }
    Err(Error::StoreError)
}

#[command]
pub async fn update_item(selected_item : Item, connection : AppHandle<Wry>) -> Result<()> {
    if let Ok(store) = get_store_from_state(connection) {
        return store.update_item(selected_item).await;
    }
    Err(Error::StoreError)
}

#[command]
pub async fn submit_board(player_one_champs : Vec<PlacedChampion>, player_two_champs : Vec<PlacedChampion>, time_unit : i8, time_till_draw: u32, connection : AppHandle<Wry>) -> Result<()> {
    Ok(())
}