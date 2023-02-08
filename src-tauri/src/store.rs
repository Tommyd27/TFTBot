use std::collections::BTreeMap;

use crate::prelude::*;
use crate::simulator::board::Board;
use crate::simulator::champions::PlacedChampion;
use crate::simulator::{
    champions::Champion, champions::DEFAULT_CHAMPIONS, item::Item, item::DEFAULT_ITEMS,
};
use surrealdb::sql::{Object, Value};
use surrealdb::{Datastore, Response, Session};
use std::mem::{swap};
use std::collections::VecDeque;
///Holds connection to database, a board and the last board simulated
pub struct Store {
    ///database file
    ds: Datastore,
    ///session of database
    ses: Session,
    ///board currently being simulated
    board : Option<Board>,
    ///last board simulated
    last_board : Option<String>
}

impl Store {
    ///Creates a new store
    pub async fn new() -> Result<Self> {
        let ds = Datastore::new("file://tft_bot_database").await.unwrap(); //opens or creates database file
        let ses = Session::for_db("appns", "appdb"); //creates a new session
        Ok(Store { ds, ses, board : None, last_board : None})
    }
    ///setups the board
    pub async fn setup(&self) -> Result<()> {
        //if there are no champions in the database
        if self.fetch_champions_ids().await?.is_empty() {
            //insert default champions
            for champ in DEFAULT_CHAMPIONS {
                match self.insert_champion(&champ).await {
                    Ok(()) => info!("successfully inserted champ: {}", champ.id),
                    Err(e) => error!("error inserting champ: {}. {}", champ.id, e),
                }
            }
        }
        //if there are no items in the database
        if self.fetch_items_ids().await?.is_empty() {
            //insert default items
            for item in DEFAULT_ITEMS {
                match self.insert_item(&item).await {
                    Ok(()) => info!("successfully inserted item: {}", item.id),
                    Err(e) => error!("error inserting item: {}. {}", item.id, e),
                }
            }
        }
		Ok(())
    }
    ///insert a champion into the database
    pub async fn insert_champion(&self, champion: &Champion) -> Result<()> {
        //create the sql statement, creating a champion with id id and content data.
        let sql = format!("CREATE champions:{id} CONTENT $data", id = champion.id);

        //turn the champion into values and then store it in a BTreeMap
        let data: BTreeMap<String, Value> = champion.into_values().into();
        let vars: BTreeMap<String, Value> = [("data".into(), data.into())].into();

        //execute on the database
        self.ds.execute(&sql, &self.ses, Some(vars), false).await?;
        Ok(())
    }
    ///insert an item into the database
    pub async fn insert_item(&self, item: &Item) -> Result<()> {
        //create the sql statement
        let sql = format!("CREATE items:{id} CONTENT $data", id = item.id);

        //turn the item into values
        let data: BTreeMap<String, Value> = item.into_values().into();
        let vars: BTreeMap<String, Value> = [("data".into(), data.into())].into();
        self.ds.execute(&sql, &self.ses, Some(vars), false).await?;
        Ok(())
    }
    ///fetch a list of all champions
    pub async fn fetch_champions(&self) -> Result<Vec<Champion>> {
        //create the sql statement
        let sql = "SELECT * FROM champions";
        //get the ress
        let ress = self.ds.execute(sql, &self.ses, None, false).await?;
        //turn it into an iterator of object, and for each object try to create a champion from it, then collect the iterator into a vector
        Ok(into_iter_objects(ress)?
            .map(|f| Champion::try_from(f.unwrap()).unwrap())
            .collect())
    }
    ///fetch a champion from an id
    pub async fn fetch_champion_from_id(&self, id: u8) -> Result<Option<Champion>> {
        let sql = &format!("SELECT * FROM champions:{id}");
        let ress = self.ds.execute(sql, &self.ses, None, false).await?;
        //if there is an object in the result vector, try to create a champion from the object
		if let Some(obj) = into_iter_objects(ress)?.next() {
			return Ok(Some(Champion::try_from(obj?)?))
		}
		Ok(None)
        
    }
    ///fetch an item from the database by id
	pub async fn fetch_item_from_id(&self, id: u8) -> Result<Option<Item>> {
        let sql = &format!("SELECT * FROM items:{id}");
        let ress = self.ds.execute(sql, &self.ses, None, false).await?;
        //if there is an object in the result vector, try to create an item from the object
		if let Some(obj) = into_iter_objects(ress)?.next() {
			return Ok(Some(Item::try_from(obj?)?))
		}
		Ok(None)
        
    }
    ///fetch a vector of all the champion ids
    pub async fn fetch_champions_ids(&self) -> Result<Vec<u8>> {
        let sql = "SELECT id FROM champions";
        //execute the statement, turn the result into a vector of objects, and for each one fetch the id and turn it onto a u8
        Ok(
            into_iter_objects(self.ds.execute(sql, &self.ses, None, false).await?)?
                .map(|f| {
                    Value::from(f.unwrap().remove("id").unwrap().record().unwrap().id).as_int()
                        as u8
                })
                .collect(),
        )
    }
    ///fetch a vector of all the item ids
    pub async fn fetch_items_ids(&self) -> Result<Vec<u8>> {
        let sql = "SELECT id FROM items";
        //execute the statement, turn the result into a vector of objects, and for each one fetch the id and turn it onto a u8
        Ok(
            into_iter_objects(self.ds.execute(sql, &self.ses, None, false).await?)?
                .map(|f| {
                    Value::from(f.unwrap().remove("id").unwrap().record().unwrap().id).as_int()
                        as u8
                })
                .collect(),
        )
    }
    ///fetch a vector of all items
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

        self.ds.execute(&sql, &self.ses, Some(vars), false).await?;
        Ok(())
    }
    pub async fn update_item(&self, item : Item) -> Result<()> {
        let sql = format!("UPDATE items:{id} CONTENT $data", id = item.id);
        let data: BTreeMap<String, Value> = item.into_values().into();
        let vars: BTreeMap<String, Value> = [("data".into(), data.into())].into();

        self.ds.execute(&sql, &self.ses, Some(vars), false).await?;
        Ok(())
    }
    pub fn set_board(&mut self, board : Board) -> Result<()> {
        self.board = Some(board);
        Ok(())
    }
    pub fn replace_board(&mut self, mut board : Option<Board>) -> Result<Option<Board>> {
        swap(&mut self.board, &mut board);
        Ok(board)
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
            data.insert("team".into(), 1.into());
            let vars: BTreeMap<String, Value> = [("data".into(), data.into())].into();
            self.ds.execute(sql, &self.ses, Some(vars), false).await?;
        }
        for champ in p2_champs {
            let mut data: BTreeMap<String, Value> = champ.into_values().into();
            data.insert("board".into(), board_link.clone().into());
            data.insert("team".into(), 2.into());
            let vars: BTreeMap<String, Value> = [("data".into(), data.into())].into();
            self.ds.execute(sql, &self.ses, Some(vars), false).await?;
        }
        self.last_board = Some(id);
        Ok(())
    }
    pub async fn update_outcome(&self, outcome : u8) -> Result<()> {
        if self.last_board.is_some() {
            let last_board = self.last_board.clone().unwrap();
            let sql = &format!("UPDATE boards:{last_board} SET outcome = {outcome}");
            self.ds.execute(sql, &self.ses, None, false).await?;
            return Ok(())
        }
        Err(Error::LastBoardError)
    }

    pub async fn fetch_outcomes(&self) -> Result<Vec<(i64, String)>> {
        let sql = "SELECT * FROM boards";
        let ress = self.ds.execute(sql, &self.ses, None, false).await?;


        Ok(into_iter_objects(ress)?.map(|obj| {
            let mut obj = obj.unwrap();
            (obj.remove("outcome").unwrap().as_int(), Value::from(obj.remove("id").unwrap().record().unwrap().id).as_string())
        }).collect())
    }

    pub async fn fetch_outcome_board(&self, id : String) -> Result<Vec<PlacedChampion>> {
        let sql = &format!("SELECT * FROM boards_champ WHERE board = boards:{id}");
        let ress = self.ds.execute(sql, &self.ses, None, false).await?;
        Ok(into_iter_objects(ress)?.map(|f| PlacedChampion::try_from(f.unwrap()).unwrap()).collect())
    }


}


fn fetch_id(ress: Vec<Response>) -> String {
    Value::from(into_iter_objects(ress).unwrap().next().unwrap().unwrap().remove("id").unwrap().record().unwrap().id).as_string()
}

///code taken from: https://www.youtube.com/watch?v=iOyvum0D3LM
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
