use crate::prelude::*;
use crate::simulator::champions::PlacedChampion;
use crate::simulator::{champions::Champion, item::Item, board::Board};
use crate::store::Store;
use std::collections::VecDeque;
use std::sync::{Arc};
use tokio::sync::{RwLock};
use tauri::{command, AppHandle, Manager, Wry};

///fetches a store from the connection
fn get_store_read_from_state(connection: AppHandle<Wry>) -> Result<Arc<RwLock<Store>>> {
    Ok((*connection.state::<Arc<RwLock<Store>>>()).clone())
}

///retrieves a unit from an id
#[command]
pub async fn retrieve_unit_from_id(id: u8, connection: AppHandle<Wry>) -> Result<Option<Champion>> {
    //fetch store
    if let Ok(store) = get_store_read_from_state(connection) {
        return store.read().await.fetch_champion_from_id(id).await //get store read only, call fetch champion from id, await response
    }
    //return failed to fetch store
    Err(Error::StoreError)
}

///retrieve an item from an id
#[command]
pub async fn retrieve_item_from_id(id: u8, connection: AppHandle<Wry>) -> Result<Option<Item>> {
    //fetch store
    if let Ok(store) = get_store_read_from_state(connection) {
        return store.read().await.fetch_item_from_id(id).await //get store read only, call fetch item from id, await response
    }
    //return failed to fetch store
    Err(Error::StoreError)
}
///retrieve all units
#[command]
pub async fn retrieve_all_units(connection : AppHandle<Wry>) -> Result<Vec<Champion>> {

    if let Ok(store) = get_store_read_from_state(connection) {
        return store.read().await.fetch_champions().await 
    }
    Err(Error::StoreError)
}
///retrieve all items
#[command]
pub async fn retrieve_all_items(connection : AppHandle<Wry>) -> Result<Vec<Item>> {
    if let Ok(store) = get_store_read_from_state(connection) {
        return store.read().await.fetch_items().await
    }
    Err(Error::StoreError)
}

///retrieve all unit ids
#[command]
pub async fn retrieve_all_unit_ids(connection : AppHandle<Wry>) -> Result<Vec<u8>> {
    if let Ok(store) = get_store_read_from_state(connection) {
        return store.read().await.fetch_champions_ids().await
    }
    Err(Error::StoreError)
}
///retrieve all item ids
#[command]
pub async fn retrieve_all_item_ids(connection : AppHandle<Wry>) -> Result<Vec<u8>> {
    if let Ok(store) = get_store_read_from_state(connection) {
        return store.read().await.fetch_items_ids().await
    }
    Err(Error::StoreError)
}
///update a unit with new values
#[command]
pub async fn update_unit(selected_unit : Champion, connection : AppHandle<Wry>) -> Result<()> {
    if let Ok(store) = get_store_read_from_state(connection) {
        return store.read().await.update_champion(selected_unit).await;
    }
    Err(Error::StoreError)
}

///update an item with new values
#[command]
pub async fn update_item(selected_item : Item, connection : AppHandle<Wry>) -> Result<()> {
    if let Ok(store) = get_store_read_from_state(connection) {
        return store.read().await.update_item(selected_item).await
    }
    Err(Error::StoreError)
}

///take a board from the frontend and set the store to hold that board
#[command]
pub async fn submit_board(player_one_champs : VecDeque<PlacedChampion>, player_two_champs : VecDeque<PlacedChampion>, time_unit : i8, time_till_draw: u32, connection : AppHandle<Wry>) -> Result<()> {
    if let Ok(store) = get_store_read_from_state(connection) {
        let champs : Vec<Champion>;
        let items : Vec<Item>;
        {   
            let store_read = store.read().await; //get readable store
            champs = store_read.fetch_champions().await?; //fetch champions and items for use in Board initialisation
            items = store_read.fetch_items().await?;
        }
        let mut store_write = store.write().await; //get writable store
        store_write.store_board(&player_one_champs, &player_two_champs).await?; //store board in database
        return store_write.set_board(Board::new(&player_one_champs, &player_two_champs, &champs, &items, time_unit, time_till_draw)); //set board to new board
    }
    Err(Error::StoreError)
}

///simulate x ticks of stored board
#[command]
pub async fn simulate_x_ticks(num_ticks : Option<u32>, connection : AppHandle<Wry>) -> Result<()> {
    if let Ok(store) = get_store_read_from_state(connection) {
        let board_opt = store.write().await.replace_board(None)?; //fetch board
        if let Some(mut board) = board_opt { //if board exists
            board.simulate_battle(num_ticks); //simulate battle
            store.write().await.replace_board(Some(board))?; //swap back board
            return Ok(())
        }   
        else {
            return Err(Error::FetchBoardError) //return fetch board error
        }

    }
    Err(Error::StoreError) //return fetch store error
}

#[command]
pub async fn fetch_board(connection : AppHandle<Wry>) -> Result<Option<Board>> {
    if let Ok(store) = get_store_read_from_state(connection) {
        return store.read().await.fetch_board() //return board
    }
    Err(Error::StoreError)
}
#[command]
pub async fn update_outcome(outcome : u8, connection : AppHandle<Wry>) -> Result<()> {
    if let Ok(store) = get_store_read_from_state(connection) {
        return store.read().await.update_outcome(outcome).await //take an outcome from the frontend and update the most recently saved board with said outcome
    }
    Err(Error::StoreError)
}

#[command]
pub async fn fetch_outcomes(connection : AppHandle<Wry>) -> Result<Vec<(i64, String)>> {
    if let Ok(store) = get_store_read_from_state(connection) {
        return store.read().await.fetch_outcomes().await; //fetch all results/ board
    }
    Err(Error::StoreError)
}

#[command]
pub async fn fetch_outcome_board(id : String, connection : AppHandle<Wry>) -> Result<Vec<PlacedChampion>> {
    if let Ok(store) = get_store_read_from_state(connection) {
        return store.read().await.fetch_outcome_board(id).await; //give the result id of a board, fetch the board stored in database
    }
    Err(Error::StoreError)
}