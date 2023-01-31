use std::collections::BTreeMap;
use std::fmt::format;

use crate::prelude::*;
use crate::simulator::board::Board;
use crate::simulator::champions::PlacedChampion;
use crate::simulator::{
    champions::Champion, champions::DEFAULT_CHAMPIONS, item::Item, item::DEFAULT_ITEMS,
};
use std::sync::Arc;
use surrealdb::sql::{thing, Array, Datetime, Object, Value};
use surrealdb::{Datastore, Response, Session};
use std::mem::{swap, take};
use std::collections::VecDeque;

pub struct Store {
    ds: Datastore,
    ses: Session,
    board : Option<Board>,
    last_board : Option<String>
}

impl Store {
    pub async fn new() -> Result<Self> {
        //let ds = Datastore::new("file://temp.db").await?;
        let ds = Datastore::new("file://tft_bot_database").await.unwrap();
        let ses = Session::for_db("appns", "appdb");
        Ok(Store { ds, ses, board : None, last_board : None})
    }
    pub async fn setup(&self) -> Result<()> {
        if self.fetch_champions_ids().await?.is_empty() {
            for champ in DEFAULT_CHAMPIONS {
                match self.insert_champion(&champ).await {
                    Ok(()) => info!("successfully inserted champ: {}", champ.id),
                    Err(e) => error!("error inserting champ: {}. {}", champ.id, e),
                }
            }
        }
        if self.fetch_items_ids().await?.is_empty() {
            for item in DEFAULT_ITEMS {
                match self.insert_item(&item).await {
                    Ok(()) => info!("successfully inserted item: {}", item.id),
                    Err(e) => error!("error inserting item: {}. {}", item.id, e),
                }
            }
        }
        let champ = self.fetch_champion_from_id(27).await;
		println!("{champ:?}");
		Ok(())
    }
    pub async fn insert_champion(&self, champion: &Champion) -> Result<()> {
        let sql = format!("CREATE champions:{id} CONTENT $data", id = champion.id);
        let data: BTreeMap<String, Value> = champion.into_values().into();
        let vars: BTreeMap<String, Value> = [("data".into(), data.into())].into();

        let ress = self.ds.execute(&sql, &self.ses, Some(vars), false).await?;
        Ok(())
    }
    pub async fn insert_item(&self, item: &Item) -> Result<()> {
        let sql = format!("CREATE items:{id} CONTENT $data", id = item.id);
        let data: BTreeMap<String, Value> = item.into_values().into();
        let vars: BTreeMap<String, Value> = [("data".into(), data.into())].into();

        let ress: Vec<Result<Object>> =
            into_iter_objects(self.ds.execute(&sql, &self.ses, Some(vars), false).await?)?
                .collect();
        println!("{ress:?}");
        Ok(())
    }
    pub async fn fetch_champions(&self) -> Result<Vec<Champion>> {
        let sql = "SELECT * FROM champions";
        let ress = self.ds.execute(sql, &self.ses, None, false).await?;

        Ok(into_iter_objects(ress)?
            .map(|f| Champion::try_from(f.unwrap()).unwrap())
            .collect())
    }
    pub async fn fetch_champion_from_id(&self, id: u8) -> Result<Option<Champion>> {
        let sql = &format!("SELECT * FROM champions:{id}");
        let ress = self.ds.execute(sql, &self.ses, None, false).await?;
		if let Some(obj) = into_iter_objects(ress)?.next() {
			return Ok(Some(Champion::try_from(obj?)?))
		}
		Ok(None)
        
    }
	pub async fn fetch_item_from_id(&self, id: u8) -> Result<Option<Item>> {
        let sql = &format!("SELECT * FROM items:{id}");
        let ress = self.ds.execute(sql, &self.ses, None, false).await?;
		if let Some(obj) = into_iter_objects(ress)?.next() {
			return Ok(Some(Item::try_from(obj?)?))
		}
		Ok(None)
        
    }
    pub async fn fetch_champions_ids(&self) -> Result<Vec<u8>> {
        let sql = "SELECT id FROM champions";
        Ok(
            into_iter_objects(self.ds.execute(sql, &self.ses, None, false).await?)?
                .map(|f| {
                    Value::from(f.unwrap().remove("id").unwrap().record().unwrap().id).as_int()
                        as u8
                })
                .collect(),
        )
    }
    pub async fn fetch_items_ids(&self) -> Result<Vec<u8>> {
        let sql = "SELECT id FROM items";
        Ok(
            into_iter_objects(self.ds.execute(sql, &self.ses, None, false).await?)?
                .map(|f| {
                    Value::from(f.unwrap().remove("id").unwrap().record().unwrap().id).as_int()
                        as u8
                })
                .collect(),
        )
    }
    pub async fn fetch_items(&self) -> Result<Vec<Item>> {
        let sql = "SELECT * FROM items";
        let ress = self.ds.execute(sql, &self.ses, None, false).await?;
        //println!("{ress:?}");
        Ok(into_iter_objects(ress)?
            .map(|f| Item::try_from(f.unwrap()).unwrap())
            .collect())
    }
    pub async fn update_champion(&self, champion : Champion) -> Result<()> {
        let sql = format!("UPDATE champions:{id} CONTENT $data", id = champion.id);
        let data: BTreeMap<String, Value> = champion.into_values().into();
        let vars: BTreeMap<String, Value> = [("data".into(), data.into())].into();

        let ress = self.ds.execute(&sql, &self.ses, Some(vars), false).await?;
        Ok(())
    }
    pub async fn update_item(&self, item : Item) -> Result<()> {
        let sql = format!("UPDATE items:{id} CONTENT $data", id = item.id);
        let data: BTreeMap<String, Value> = item.into_values().into();
        let vars: BTreeMap<String, Value> = [("data".into(), data.into())].into();

        let ress = self.ds.execute(&sql, &self.ses, Some(vars), false).await?;
        Ok(())
    }
    pub fn set_board(&mut self, board : Board) -> Result<()> {
        self.board = Some(board);
        Ok(())
    }
    pub fn replace_board(&mut self, mut board : Option<Board>) -> Result<Option<Board>> {
        swap(&mut self.board, &mut board);
        return Ok(board)
    }
    pub fn fetch_board(&self) -> Result<Option<Board>> {
        Ok(self.board.as_ref().cloned())
    }
    pub async fn store_board(&mut self, p1_champs : &VecDeque<PlacedChampion>, p2_champs : &VecDeque<PlacedChampion>) -> Result<()> {
        let sql = "CREATE boards SET outcome = 0";
        let ress = self.ds.execute(sql, &self.ses, None, false).await?;
        let id = fetch_id(ress);

        let sql = "CREATE boards_champ CONTENT $data";
        let board_link = format!("boards:{id}");
        for champ in p1_champs {
            let mut data: BTreeMap<String, Value> = champ.into_values().into();
            data.insert("board".into(), board_link.clone().into());
            let vars: BTreeMap<String, Value> = [("data".into(), data.into())].into();
            let ress = self.ds.execute(sql, &self.ses, Some(vars), false).await?;
        }
        for champ in p2_champs {
            let mut data: BTreeMap<String, Value> = champ.into_values().into();
            data.insert("board".into(), board_link.clone().into());
            let vars: BTreeMap<String, Value> = [("data".into(), data.into())].into();
            let ress = self.ds.execute(sql, &self.ses, Some(vars), false).await?;
        }
        self.last_board = Some(id);
        Ok(())
    }
    pub async fn update_outcome(&self, outcome : u8) -> Result<()> {
        if self.last_board.is_some() {
            let last_board = self.last_board.clone().unwrap();
            let sql = &format!("UPDATE boards:{last_board} SET outcome = {outcome}");
            let ress = self.ds.execute(sql, &self.ses, None, false).await?;
            return Ok(())
        }
        Err(Error::LastBoardError)
    }

    pub async fn fetch_outcomes(&self) -> Result<()> {
        let sql = "SELECT * FROM boards";
        let ress = self.ds.execute(sql, &self.ses, None, false).await?;
        for obj in into_iter_objects(ress)? {
            println!("{obj:?}");
        }
        Ok(())
    }

}


fn fetch_id(ress: Vec<Response>) -> String {
    Value::from(into_iter_objects(ress).unwrap().next().unwrap().unwrap().remove("id").unwrap().record().unwrap().id).as_string()
}
fn into_iter_objects(ress: Vec<Response>) -> Result<impl Iterator<Item = Result<Object>>> {
    let res = ress
        .into_iter()
        .next()
        .map(|rp: Response| rp.result)
        .transpose()?;
    match res {
        Some(Value::Array(arr)) => {
            let it = arr.into_iter().map(|v| match v {
                Value::Object(object) => Ok(object),
                _ => Err(Error::DatabaseError("A record was not an object")),
            });
            Ok(it)
        }
        _ => Err(Error::DatabaseError("No records found")),
    }
}
