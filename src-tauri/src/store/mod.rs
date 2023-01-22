use std::collections::BTreeMap;
use std::str::FromStr;

use surrealdb::sql::{thing, Array, Datetime, Object, Value};
use surrealdb::{Datastore, Session, Response};
use std::sync::Arc;
use crate::simulator::{champions::Champion, champions::DEFAULT_CHAMPIONS, item::Item, item::DEFAULT_ITEMS};
use crate::prelude::*;

pub struct Store
{
	ds : Datastore,
	ses: Session,
}

impl Store
{
	pub async fn new() -> Result<Self> {
		//let ds = Datastore::new("file://temp.db").await?;
		let ds = Datastore::new("file://tft_bot_database").await.unwrap();
		let ses = Session::for_db("appns", "appdb");
		Ok(Store {ds, ses})
	}
	pub async fn setup(&self) -> Result<()> {
		if self.fetch_champions_ids().await?.is_empty() {
			for champ in DEFAULT_CHAMPIONS {
				match self.insert_champion(&champ).await {
					Ok(()) => info!("successfully inserted champ: {}", champ.id),
					Err(e) => error!("error inserting champ: {}. {}", champ.id, e)
				}
			}
		}
		if self.fetch_items_ids().await?.is_empty() {
			for item in DEFAULT_ITEMS {
				match self.insert_item(&item).await {
					Ok(()) => info!("successfully inserted item: {}", item.id),
					Err(e) => error!("error inserting item: {}. {}", item.id, e)
				}
			}
		}
		Ok(())
	}
	pub async fn insert_champion(&self, champion : &Champion) -> Result<()> {

		let sql = format!("CREATE champions:{id} CONTENT $data", id = champion.id);
	   	let data : BTreeMap<String, Value> = champion.into_values().into();
	   	let vars : BTreeMap<String, Value> = [("data".into(), data.into())].into();

	   	let ress = self.ds.execute(&sql, &self.ses, Some(vars), false).await?;
	   	println!("{ress:?}");
		Ok(())
   }
	pub async fn insert_item(&self, item : &Item) -> Result<()>{

 		let sql = format!("CREATE items:{id} CONTENT $data", id = item.id);
		let data : BTreeMap<String, Value> = item.into_values().into();
		let vars : BTreeMap<String, Value> = [("data".into(), data.into())].into();

		let ress : Vec<Result<Object>> = into_iter_objects(self.ds.execute(&sql, &self.ses, Some(vars), false).await?)?.collect();
		println!("{ress:?}");
		Ok(())
	}
	pub async fn fetch_champions(&self) -> Result<Vec<Champion>>{
		let sql = "SELECT * FROM champions";
		let ress = self.ds.execute(sql, &self.ses, None, false).await?;

		Ok(into_iter_objects(ress)?.map(|f| Champion::try_from(f.unwrap()).unwrap()).collect())
	}

	pub async fn fetch_champions_ids(&self) -> Result<Vec<u8>> {
		let sql = "SELECT id FROM champions";
		Ok(into_iter_objects(self.ds.execute(sql, &self.ses, None, false).await?)?.map(|f| Value::from(f.unwrap().remove("id").unwrap().record().unwrap().id).as_int() as u8).collect())
	}
	pub async fn fetch_items_ids(&self) -> Result<Vec<u8>> {
		let sql = "SELECT id FROM items";
		Ok(into_iter_objects(self.ds.execute(sql, &self.ses, None, false).await?)?.map(|f| Value::from(f.unwrap().remove("id").unwrap().record().unwrap().id).as_int() as u8).collect())
	}
	pub async fn fetch_items(&self) -> Result<Vec<Item>> {
		let sql = "SELECT * FROM items";
		let ress = self.ds.execute(sql, &self.ses, None, false).await?;
		//println!("{ress:?}");
		Ok(into_iter_objects(ress)?.map(|f| Item::try_from(f.unwrap()).unwrap()).collect())
	}
}

fn into_iter_objects(ress : Vec<Response>) -> Result<impl Iterator<Item = Result<Object>>> {
	let res = ress.into_iter().next().map(|rp : Response| rp.result).transpose()?;
	match res {
		Some(Value::Array(arr)) => {
			let it = arr.into_iter().map(|v| match v {
				Value::Object(object) => Ok(object),
				_ => Err(Error::DatabaseError("A record was not an object"))
			});
			Ok(it)
		},
		_ => Err(Error::DatabaseError("No records found"))
	}
}