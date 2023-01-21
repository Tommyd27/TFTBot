use std::collections::BTreeMap;
use std::str::FromStr;

use surrealdb::sql::{thing, Array, Datetime, Object, Value};
use surrealdb::{Datastore, Session, Response};
use std::sync::Arc;
use crate::simulator::Champion;

pub struct Store
{
	ds : Datastore,
	ses: Session,
}

impl Store
{
	pub async fn new() -> Self
	{
		//let ds = Datastore::new("file://temp.db").await?;
		let ds = Datastore::new("file://tft_bot_database").await.unwrap();
		let ses = Session::for_db("appns", "appdb");
		Store {ds, ses}
	}
}