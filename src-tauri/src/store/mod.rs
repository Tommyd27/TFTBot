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
		self.fetch_champions().await;
		self.fetch_items().await;
		if false {
			for champ in DEFAULT_CHAMPIONS {
				self.insert_champion(&champ).await;
			}
		}
		if false {
			for item in DEFAULT_ITEMS {
				self.insert_item(&item).await;
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

		let ress = self.ds.execute(&sql, &self.ses, Some(vars), false).await?;
		println!("{ress:?}");
		Ok(())
	}
	pub async fn fetch_champions(&self) -> Result<()>{
		let sql = "SELECT * FROM champions";
		let ress = self.ds.execute(sql, &self.ses, None, false).await?;
		//println!("{ress:?}");

		let into_iter : Vec<Result<Object>> = into_iter_objects(ress)?.collect();
		//println!("{into_iter:?}");

		for obj in into_iter {
			let obj = obj?;
			let champ = Champion::try_from(obj);
			break
		}

		Ok(())
	}
	pub async fn fetch_items(&self) -> Result<()> {
		let sql = "SELECT * FROM champions";
		let ress = self.ds.execute(sql, &self.ses, None, false).await?;
		//println!("{ress:?}");
		Ok(())
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