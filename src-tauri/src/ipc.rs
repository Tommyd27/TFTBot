use crate::prelude::*;
use crate::simulator::champions::PlacedChampion;
use crate::simulator::{champions::Champion, item::Item, board::Board};
use crate::store::Store;
use std::collections::VecDeque;
use std::sync::{Arc};
use tokio::sync::{RwLock, RwLockReadGuard};
use tauri::{command, AppHandle, Manager, Wry};
use std::mem::replace;

fn get_store_read_from_state(connection: AppHandle<Wry>) -> Result<Arc<RwLock<Store>>> {
    Ok((*connection.state::<Arc<RwLock<Store>>>()).clone())
}

#[command]
pub async fn retrieve_unit_from_id(id: u8, connection: AppHandle<Wry>) -> Result<Option<Champion>> {
    if let Ok(store) = get_store_read_from_state(connection) {
        return store.read().await.fetch_champion_from_id(id).await
    }
    Err(Error::StoreError)
}

#[command]
pub async fn retrieve_item_from_id(id: u8, connection: AppHandle<Wry>) -> Result<Option<Item>> {
    //*Option<Champion> {
    if let Ok(store) = get_store_read_from_state(connection) {
        return store.read().await.fetch_item_from_id(id).await
    }
    Err(Error::StoreError)
}

#[command]
pub async fn retrieve_all_units(connection : AppHandle<Wry>) -> Result<Vec<Champion>> {
    if let Ok(store) = get_store_read_from_state(connection) {
        return store.read().await.fetch_champions().await
    }
    Err(Error::StoreError)
}

#[command]
pub async fn retrieve_all_items(connection : AppHandle<Wry>) -> Result<Vec<Item>> {
    if let Ok(store) = get_store_read_from_state(connection) {
        return store.read().await.fetch_items().await
    }
    Err(Error::StoreError)
}
#[command]
pub async fn retrieve_all_unit_ids(connection : AppHandle<Wry>) -> Result<Vec<u8>> {
    if let Ok(store) = get_store_read_from_state(connection) {
        return store.read().await.fetch_champions_ids().await
    }
    Err(Error::StoreError)
}
#[command]
pub async fn retrieve_all_item_ids(connection : AppHandle<Wry>) -> Result<Vec<u8>> {
    if let Ok(store) = get_store_read_from_state(connection) {
        return store.read().await.fetch_items_ids().await
    }
    Err(Error::StoreError)
}

#[command]
pub async fn update_unit(selected_unit : Champion, connection : AppHandle<Wry>) -> Result<()> {
    if let Ok(store) = get_store_read_from_state(connection) {
        return store.read().await.update_champion(selected_unit).await;
    }
    Err(Error::StoreError)
}

#[command]
pub async fn update_item(selected_item : Item, connection : AppHandle<Wry>) -> Result<()> {
    if let Ok(store) = get_store_read_from_state(connection) {
        return store.read().await.update_item(selected_item).await
    }
    Err(Error::StoreError)
}

#[command]
pub async fn submit_board(player_one_champs : VecDeque<PlacedChampion>, player_two_champs : VecDeque<PlacedChampion>, time_unit : i8, time_till_draw: u32, connection : AppHandle<Wry>) -> Result<()> {
    if let Ok(store) = get_store_read_from_state(connection) {
        let champs : Vec<Champion>;
        let items : Vec<Item>;
        {   
            let store_read = store.read().await;
            champs = store_read.fetch_champions().await?;
            items = store_read.fetch_items().await?;
        }
        let mut store_write = store.write().await;
        return store_write.set_board(Board::new(&player_one_champs, &player_two_champs, &champs, &items, time_unit, time_till_draw));
    }
    Err(Error::StoreError)
}

#[command]
pub async fn simulate_x_ticks(num_ticks : Option<u32>, connection : AppHandle<Wry>) -> Result<()> {
    if let Ok(store) = get_store_read_from_state(connection) {
        let board_opt = store.write().await.replace_board(None)?;
        if let Some(mut board) = board_opt {
            board.simulate_battle(num_ticks);
            store.write().await.replace_board(Some(board))?;
            return Ok(())
        }   
        else {
            return Err(Error::FetchBoardError)
        }

    }
    Err(Error::StoreError)
}

#[command]
pub async fn fetch_board(connection : AppHandle<Wry>) -> Result<Option<Board>> {
    if let Ok(store) = get_store_read_from_state(connection) {
        return store.read().await.fetch_board()
    }
    Err(Error::StoreError)
}