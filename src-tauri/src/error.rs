use serde::Serialize;

///All possible errors for this project (not related to simulator)
#[derive(thiserror::Error, Debug, Serialize)]
pub enum Error {
	///Error from SurrealDB
	#[error(transparent)]
	Surreal(#[from] surrealdb::Error),

	///Error related to database
	#[error("Database Error: '{0}'")]
	DatabaseError(&'static str),

	///Error retrieving Store from Ctx/ Tauri
	#[error("Failed to retrieve store")]
	StoreError,

	///Error fetching Board from Store
	#[error("Failed to fetch board")]
	FetchBoardError,

	///Error fetching last board from Store
	#[error("Failed to retrieve last board")]
	LastBoardError,
	
}